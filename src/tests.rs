use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

// -------------------------------------------
// works
#[test]
fn init_letter_works() {
	new_test_ext().execute_with(|| {

		assert_eq!(Letters::all_letters_count(), 0);

		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		assert_ok!(Letters::init_letter(Origin::signed(1), title.clone(), author.clone()));

		assert_eq!(Letters::all_letters_count(), 1);

        let letter_id = Letters::letter_by_index(1);

		assert_eq!(Letters::letter(letter_id).title, title);
		assert_eq!(Letters::letter(letter_id).author, author);
	});
}

#[test]
fn read_write_page_works() {
	new_test_ext().execute_with(|| {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		assert_ok!(Letters::init_letter(Origin::signed(1), title.clone(), author.clone()));

		let letter_id = Letters::letter_by_index(1);
		let page = "𝔯𝔬𝔰𝔢𝔰 𝔞𝔯𝔢 𝔯𝔢𝔡 🌹".as_bytes().to_vec();

		assert_ok!(Letters::write_page(Origin::signed(1), letter_id, page.clone()));
		assert_eq!(Letters::read_page(letter_id, 0).unwrap(), page);

		let page = "𝔳𝔦𝔬𝔩𝔢𝔱𝔰 𝔞𝔯𝔢 𝔟𝔩𝔲𝔢 ❃".as_bytes().to_vec();
		assert_ok!(Letters::write_page(Origin::signed(1), letter_id, page.clone()));
		assert_eq!(Letters::read_page(letter_id, 1).unwrap(), page);

	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		let alice = 1;
		let bob = 2;
		let alice_signed = Origin::signed(alice);

		assert_ok!(Letters::init_letter(alice_signed.clone(), title.clone(), author.clone()));

		let letter_id = Letters::letter_by_index(1);
		let page = "𝔯𝔬𝔰𝔢𝔰 𝔞𝔯𝔢 𝔯𝔢𝔡 🌹".as_bytes().to_vec();

		assert_ok!(Letters::write_page(alice_signed.clone(), letter_id, page.clone()));

		assert_eq!(Letters::letter_of_owner_by_index((alice, 1)), letter_id);
		assert_eq!(Letters::owner_of(letter_id), Some(alice));
		assert_eq!(Letters::owned_letter_count(alice), 1);
		assert_eq!(Letters::owned_letter_count(bob), 0);

		assert_ok!(Letters::transfer(alice_signed, bob, letter_id));
		println!("{:?}", Some(bob));

		let a = Letters::owned_letter_count(alice);

		// assert_eq!(Letters::letter_of_owner_by_index((bob, 1)), letter_id);
		assert_eq!(Letters::owner_of(letter_id), Some(bob));
		assert_eq!(Letters::owned_letter_count(bob), 1);
		assert_eq!(Letters::owned_letter_count(alice), 0);

	});
}

// -------------------------------------------
// error

#[test]
fn write_page_wrong_owner_error() {
	new_test_ext().execute_with(|| {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		assert_ok!(Letters::init_letter(Origin::signed(1), title.clone(), author.clone()));

		let letter_id = Letters::letter_by_index(1);
		let page = "𝔯𝔬𝔰𝔢𝔰 𝔞𝔯𝔢 𝔯𝔢𝔡 🌹".as_bytes().to_vec();

		assert_noop!(Letters::write_page(Origin::signed(2), letter_id, page.clone()), Error::<Test>::LetterNotOwned);
		assert_noop!(Letters::read_page(letter_id, 0), Error::<Test>::NonExistentPage);
	});
}

#[test]
fn non_existent_page_error() {
	new_test_ext().execute_with(|| {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		assert_ok!(Letters::init_letter(Origin::signed(1), title.clone(), author.clone()));
		let letter_id = Letters::letter_by_index(1);
		let page = "𝔯𝔬𝔰𝔢𝔰 𝔞𝔯𝔢 𝔯𝔢𝔡 🌹".as_bytes().to_vec();

		assert_ok!(Letters::write_page(Origin::signed(1), letter_id, page.clone()));
		assert_eq!(Letters::read_page(letter_id, 0).unwrap(), page);

		assert_noop!(Letters::read_page(letter_id, 1), Error::<Test>::NonExistentPage);

		let page = "𝔳𝔦𝔬𝔩𝔢𝔱𝔰 𝔞𝔯𝔢 𝔟𝔩𝔲𝔢 ❃".as_bytes().to_vec();
		assert_ok!(Letters::write_page(Origin::signed(1), letter_id, page.clone()));
		assert_eq!(Letters::read_page(letter_id, 1).unwrap(), page);

	});
}
// -------------------------------------------
// templates

// #[test]
// fn xxx_works() {
// 	new_test_ext().execute_with(|| {
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }


// #[test]
// fn xxx_error() {
// 	new_test_ext().execute_with(|| {
//  		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }