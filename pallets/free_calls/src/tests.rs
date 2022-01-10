use crate::test_pallet::pallet as test_pallet;
use crate::mock::{self, *};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(<test_pallet::Pallet<Test>>::subtract(Origin::signed(1), 42, 12));
		let event = <frame_system::Pallet<Test>>::events().pop()
			.expect("Expected Result EventRecord").event;
		assert_eq!(event, mock::Event::from(test_pallet::Event::Result(42 - 12)));
	});
}
