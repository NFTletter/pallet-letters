use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BuildStorage;

const ALICE_INIT_BALANCE: u128 = 1000;
const BOB_INIT_BALANCE: u128 = 2000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = GenesisConfig {
		system: frame_system::GenesisConfig::default(),
		balances: pallet_balances::GenesisConfig {
			balances: vec![(1, ALICE_INIT_BALANCE), (2, BOB_INIT_BALANCE)],
		},
	}
	.build_storage()
	.unwrap()
	.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

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
		let (letter, _) = Letters::letter(letter_id).unwrap();

		assert_eq!(letter.title, title);
		assert_eq!(letter.author, author);

		assert_eq!(
			Balances::reserved_balance(&1),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128
		);

		let title2 = "hello world".as_bytes().to_vec();
		let author2 = "bear".as_bytes().to_vec();

		assert_ok!(Letters::init_letter(Origin::signed(1), title2.clone(), author2.clone()));

		assert_eq!(Letters::all_letters_count(), 2);

		let letter_id = Letters::letter_by_index(2);
		let (letter, _) = Letters::letter(letter_id).unwrap();

		assert_eq!(letter.title, title2);
		assert_eq!(letter.author, author2);

		assert_eq!(
			Balances::reserved_balance(&1),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				LETTER_DEPOSIT_BASE as u128 +
				(title2.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author2.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128
		);
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
		let bob_signed = Origin::signed(bob);

		assert_ok!(Letters::init_letter(alice_signed.clone(), title.clone(), author.clone()));

		let letter_id = Letters::letter_by_index(1);
		let page = "𝔯𝔬𝔰𝔢𝔰 𝔞𝔯𝔢 𝔯𝔢𝔡 🌹".as_bytes().to_vec();

		assert_ok!(Letters::write_page(alice_signed.clone(), letter_id, page.clone()));

		assert_eq!(
			Balances::reserved_balance(&alice),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				PAGE_DEPOSIT_BASE as u128 +
				(page.len() as u32 * PAGE_DEPOSIT_FACTOR) as u128
		);
		assert_eq!(Letters::letter_of_owner_by_index((alice, 1)), letter_id);
		assert_eq!(Letters::owner_of(letter_id), Some(alice));
		assert_eq!(Letters::owned_letter_count(alice), 1);
		assert_eq!(Letters::owned_letter_count(bob), 0);

		assert_ok!(Letters::transfer(alice_signed, bob, letter_id));

		assert_eq!(
			Balances::reserved_balance(&bob),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				PAGE_DEPOSIT_BASE as u128 +
				(page.len() as u32 * PAGE_DEPOSIT_FACTOR) as u128
		);

		assert_eq!(Letters::letter_of_owner_by_index((bob, 1)), letter_id);
		assert_eq!(Letters::owner_of(letter_id), Some(bob));
		assert_eq!(Letters::owned_letter_count(bob), 1);
		assert_eq!(Letters::owned_letter_count(alice), 0);

		let page2 = "𝔳𝔦𝔬𝔩𝔢𝔱𝔰 𝔞𝔯𝔢 𝔟𝔩𝔲𝔢 ❃".as_bytes().to_vec();
		assert_ok!(Letters::write_page(bob_signed.clone(), letter_id, page2.clone()));
		assert_eq!(Letters::read_page(letter_id, 1).unwrap(), page2);
		assert_eq!(
			Balances::reserved_balance(&bob),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				PAGE_DEPOSIT_BASE as u128 +
				(page.len() as u32 * PAGE_DEPOSIT_FACTOR) as u128 +
				PAGE_DEPOSIT_BASE as u128 +
				(page2.len() as u32 * PAGE_DEPOSIT_FACTOR) as u128
		);
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

		assert_noop!(
			Letters::write_page(Origin::signed(2), letter_id, page.clone()),
			Error::<Test>::LetterNotOwned
		);
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
		assert_eq!(Letters::read_page(letter_id, 0).unwrap().into_inner(), page);

		assert_noop!(Letters::read_page(letter_id, 1), Error::<Test>::NonExistentPage);

		let page = "𝔳𝔦𝔬𝔩𝔢𝔱𝔰 𝔞𝔯𝔢 𝔟𝔩𝔲𝔢 ❃".as_bytes().to_vec();
		assert_ok!(Letters::write_page(Origin::signed(1), letter_id, page.clone()));
		assert_eq!(Letters::read_page(letter_id, 1).unwrap(), page);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		let alice = 1;
		let bob = 2;
		let alice_signed = Origin::signed(alice);
		let bob_signed = Origin::signed(bob);

		assert_eq!(Balances::free_balance(&alice), ALICE_INIT_BALANCE);
		assert_eq!(Balances::free_balance(&bob), BOB_INIT_BALANCE);

		assert_ok!(Letters::init_letter(alice_signed.clone(), title.clone(), author.clone()));
		let letter_id = Letters::letter_by_index(1);
		assert_eq!(
			Balances::free_balance(&alice),
			ALICE_INIT_BALANCE -
				LETTER_DEPOSIT_BASE as u128 -
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 -
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128
		);

		let price = 500u32;
		assert_ok!(Letters::set_price(alice_signed.clone(), letter_id, price.into()));

		assert_ok!(Letters::buy_letter(bob_signed, letter_id, 500u32.into()));
		assert_eq!(
			Balances::reserved_balance(&bob),
			LETTER_DEPOSIT_BASE as u128 +
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 +
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128
		);
		assert_eq!(
			Balances::free_balance(&bob),
			BOB_INIT_BALANCE -
				(title.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 -
				(author.len() as u32 * LETTER_DEPOSIT_FACTOR) as u128 -
				LETTER_DEPOSIT_BASE as u128 -
				price as u128
		);
	});
}
