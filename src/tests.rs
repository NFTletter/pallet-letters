use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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
fn write_page_works() {
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
//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
