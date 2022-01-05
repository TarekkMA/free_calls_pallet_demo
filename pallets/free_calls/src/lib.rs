#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_support::weights::GetDispatchInfo;
	use frame_system::pallet_prelude::*;
	use frame_system::RawEvent;
	use sp_runtime::traits::Dispatchable;
	use sp_std::boxed::Box;
	use sp_std::vec::Vec;
	use scale_info::TypeInfo;


	pub type NumberOfCalls = u16;
	pub type WindowConfigsSize = u8;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct WindowStats<BlockNumber> {
		pub index: BlockNumber,
		pub num_of_calls: NumberOfCalls,
	}

	impl<BlockNumber> WindowStats<BlockNumber> {
		fn new(window_index: BlockNumber) -> Self {
			WindowStats {
				index: window_index,
				num_of_calls: 0,
			}
		}
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct WindowConfig<BlockNumber> {
		pub period: BlockNumber,
		pub max_num_of_calls: NumberOfCalls,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Call: Parameter + GetDispatchInfo + Dispatchable<Origin=Self::Origin>;

		#[pallet::constant]
		type WindowsConfig: Get<Vec<WindowConfig<Self::BlockNumber>>>;
	}


	#[pallet::storage]
	#[pallet::getter(fn window_stats_by_account)]
	pub(super) type WindowStatsByAccount<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Twox64Concat,
		WindowConfigsSize,
		WindowStats<T::BlockNumber>,
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// free call was executed. [who, result]
		FreeCallResult(T::AccountId, DispatchResult),
		Test(u32),
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn try_free_call(origin: OriginFor<T>, call: Box<<T as Config>::Call>) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			if Self::can_make_free_call_and_update_stats(&sender) {

				// Dispatch the call
				let result = call.dispatch(origin);

				// Deposit an event with the result
				Self::deposit_event(
					Event::FreeCallResult(
						sender,
						result.map(|_| ()).map_err(|e| e.error),
					)
				);
			}

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn test(origin: OriginFor<T>, n1: u32, n2: u32) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			Self::deposit_event(Event::Test(n1 + n2));

			Ok(())
		}
	}


	struct Window<T: Config> {
		account: T::AccountId,
		config_index: WindowConfigsSize,
		config: WindowConfig<T::BlockNumber>,
		window_index: T::BlockNumber,
		stats: WindowStats<T::BlockNumber>,
		can_be_called: bool,
	}

	impl<T: Config> Window<T> {
		fn build(
			account: T::AccountId,
			current_block: T::BlockNumber,
			config_index: WindowConfigsSize,
			config: WindowConfig<T::BlockNumber>,
			window_stats: Option<WindowStats<T::BlockNumber>>,
		) -> Self {
			let config_index = config_index as WindowConfigsSize;

			let window_index = current_block / config.period;

			let reset_stats = || { WindowStats::new(current_block) };

			let mut stats = window_stats.unwrap_or_else(reset_stats);

			if stats.index < window_index {
				stats = reset_stats();
			}

			let can_be_called = stats.num_of_calls < config.max_num_of_calls;

			Window {
				account: account.clone(),
				config_index,
				config,
				window_index,
				stats,
				can_be_called,
			}
		}

		fn increment_window_stats(&mut self) {
			self.stats.num_of_calls.saturating_add(1);
			<WindowStatsByAccount<T>>::insert(self.account.clone(), self.config_index, self.stats.clone());
		}
	}

	impl<T: Config> Pallet<T> {
		fn can_make_free_call_and_update_stats(account: &T::AccountId) -> bool {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let windows_config = T::WindowsConfig::get();


			let mut windows: Vec<Window<T>> = Vec::new();
			let mut can_call = false;

			for (config_index, config) in windows_config
				.into_iter()
				.enumerate() {
				let config_index = config_index as WindowConfigsSize;
				let window = Window::build(
					account.clone(),
					current_block,
					config_index,
					config,
					Self::window_stats_by_account(account.clone(), config_index),
				);
				can_call = window.can_be_called;
				if !can_call {
					break;
				}
				windows.push(window);
			}

			if can_call {
				for window in &mut windows {
					window.increment_window_stats();
				}
			}

			can_call
		}
	}
}
