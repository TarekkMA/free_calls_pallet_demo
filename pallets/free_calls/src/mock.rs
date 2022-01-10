use crate as pallet_free_calls;
use crate::test_pallet::pallet as test_pallet;
use frame_support::parameter_types;
use frame_system as system;
use frame_system::EventRecord;
use sp_core::H256;
use sp_runtime::{MultiSignature, testing::Header, traits::{BlakeTwo256, IdentityLookup}};
use sp_runtime::traits::{IdentifyAccount, Verify};
use pallet_free_calls::WindowConfig;

/// An index to a block.
pub type BlockNumber = u64;
pub type Hash = sp_core::H256;


/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TestPallet: test_pallet::{Pallet, Call, Storage, Event<T>},
		// FreeCalls: pallet_free_calls::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

impl test_pallet::Config for Test {
	type Event = Event;
}

parameter_types! {
	pub FreeCallsWindowsConfig: Vec<WindowConfig<BlockNumber>> = [
		WindowConfig {
			period: 10,
			quota_ratio: 1,
		}
	].to_vec();
}

// impl pallet_free_calls::Config for Test {
// 	type Event = Event;
// 	type Call = Call;
// 	type WindowsConfig = FreeCallsWindowsConfig;
// 	// TODO change this
// 	type ManagerOrigin = frame_system::EnsureRoot<AccountId>;
// }

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| <frame_system::Pallet<Test>>::set_block_number(1));
	ext
}
