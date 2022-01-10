
#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, log, pallet_prelude::*};
	use frame_support::weights::GetDispatchInfo;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Dispatchable;
	use sp_std::boxed::Box;
	use sp_std::vec::Vec;
	use scale_info::TypeInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Result(i32),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn subtract(origin: OriginFor<T>, n1: i32, n2: i32) -> DispatchResult {
			Self::deposit_event(Event::Result(n1 - n2));

			Ok(())
		}
	}
}
