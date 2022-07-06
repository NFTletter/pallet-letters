//! Benchmarking setup for pallet-letters

use super::*;

#[allow(unused)]
use crate::Pallet as Letters;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller, Vec};
use frame_support::traits::{Currency, Get};
use frame_system::RawOrigin;

fn create_vec(n: u32) -> Vec<u8> {
	let mut v = Vec::new();
	for _ in 0..n {
		v.push(b'.');
	}
	v
}

benchmarks! {
	init_letter {
		let t in 0 .. T::MaxTitleLength::get() as u32;
		let a in 0 .. T::MaxAuthorLength::get() as u32;

		let title = create_vec(t);
		let author = create_vec(a);

		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), title, author)
	verify {
		let title = create_vec(t);
		let author = create_vec(a);

		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		assert_eq!(Letters::<T>::letter(letter_id).unwrap().title, title);
		assert_eq!(Letters::<T>::letter(letter_id).unwrap().author, author);
	}

	write_page {
		let p in 0 .. T::MaxPageLength::get() as u32;
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		let caller: T::AccountId = whitelisted_caller();

		Letters::<T>::init_letter(RawOrigin::Signed(caller.clone()).into(), title, author)?;

		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		let page = create_vec(p);
	}: _(RawOrigin::Signed(caller), letter_id, page)
	verify {
		let page = create_vec(p);
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		assert_eq!(Letters::<T>::read_page(letter_id, 0).unwrap(), page);
	}

	set_price {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();

		let caller: T::AccountId = whitelisted_caller();
		Letters::<T>::init_letter(RawOrigin::Signed(caller.clone()).into(), title, author)?;

		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		let new_price: T::Balance = 10u32.into();

	}: _(RawOrigin::Signed(caller), letter_id, new_price)
	verify {
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		assert_eq!(Letters::<T>::letter(letter_id).unwrap().price, new_price);
	}

	transfer {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();
		let alice: T::AccountId = account("Alice", 0, 0);
		let bob: T::AccountId = account("Bob", 0, 1);

		Letters::<T>::init_letter(RawOrigin::Signed(alice.clone()).into(), title, author)?;
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);

	}: _(RawOrigin::Signed(alice), bob.clone(), letter_id)
	verify {
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		assert_eq!(Letters::<T>::letter_of_owner_by_index((bob, 1)), letter_id);
	}

	buy_letter {
		let title = "𝔥𝔢𝔩𝔩𝔬 𝔴𝔬𝔯𝔩𝔡".as_bytes().to_vec();
		let author = "𝖇𝖊𝖆𝖗".as_bytes().to_vec();
		let alice: T::AccountId = account("Alice", 0, 0);
		let bob: T::AccountId = account("Bob", 0, 1);

		T::Currency::make_free_balance_be(&bob, 1000u32.into());

		Letters::<T>::init_letter(RawOrigin::Signed(alice.clone()).into(), title, author)?;
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		Letters::<T>::set_price(RawOrigin::Signed(alice.clone()).into(), letter_id, 500u32.into())?;

	}: _(RawOrigin::Signed(bob.clone()), letter_id, 500u32.into())
	verify {
		let letter_id: T::Hash = Letters::<T>::letter_by_index(1);
		assert_eq!(Letters::<T>::letter_of_owner_by_index((bob, 1)), letter_id);
	}
}

impl_benchmark_test_suite!(Letters, crate::mock::new_test_ext(), crate::mock::Test);
