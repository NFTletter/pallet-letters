#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
use frame_support::traits::Currency;
pub use weights::*;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		sp_runtime::traits::{Hash, Zero},
		traits::{ExistenceRequirement, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{prelude::vec::Vec, TypeInfo};

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Letter<T: Config> {
		pub id: T::Hash,
		pub title: BoundedVec<u8, T::MaxTitleLength>,
		pub author: BoundedVec<u8, T::MaxAuthorLength>,
		pub price: T::Balance,
		pages: BoundedVec<BoundedVec<u8, T::MaxPageLength>, T::MaxPageNum>,
	}

	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: ReservableCurrency<Self::AccountId>;
		type WeightInfo: WeightInfo;

		/// The base amount of currency needed to reserve for starting a letter.
		#[pallet::constant]
		type LetterDepositBase: Get<BalanceOf<Self>>;

		/// The amount of currency needed to reserve per byte in author and title of a letter.
		#[pallet::constant]
		type LetterDepositFactor: Get<BalanceOf<Self>>;

		/// The base amount of currency needed to reserve for adding a page.
		#[pallet::constant]
		type PageDepositBase: Get<BalanceOf<Self>>;

		/// The amount of currency needed to reserve per byte in page added.
		#[pallet::constant]
		type PageDepositFactor: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type MaxTitleLength: Get<u32>;

		#[pallet::constant]
		type MaxAuthorLength: Get<u32>;

		#[pallet::constant]
		type MaxPageLength: Get<u32>;

		#[pallet::constant]
		type MaxPageNum: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Stores a Letter: it's unique traits and price.
	#[pallet::storage]
	#[pallet::getter(fn letter)]
	pub(super) type Letters<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, (Letter<T>, BalanceOf<T>)>;

	// Keeps track of what accounts own what Letter.
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub(super) type LetterOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	// An index to track of all Letters.
	#[pallet::storage]
	#[pallet::getter(fn letter_by_index)]
	pub(super) type AllLettersArray<T: Config> =
		StorageMap<_, Twox64Concat, u64, T::Hash, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn all_letters_count)]
	pub(super) type AllLettersCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Keeps track of all the Letters.
	#[pallet::storage]
	pub(super) type AllLettersIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	// Keep track of who a Letter is owned by.
	#[pallet::storage]
	#[pallet::getter(fn letter_of_owner_by_index)]
	pub(super) type OwnedLettersArray<T: Config> =
		StorageMap<_, Twox64Concat, (T::AccountId, u64), T::Hash, ValueQuery>;

	// Keeps track of the total amount of Letters owned.
	#[pallet::storage]
	#[pallet::getter(fn owned_letter_count)]
	pub(super) type OwnedLettersCount<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	// Keeps track of all owned Letters by index.
	#[pallet::storage]
	pub(super) type OwnedLettersIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub letters: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, T::AccountId, T::Hash, T::Balance)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { letters: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (title, author, page, acct, hash, balance) in &self.letters {
				let bounded_title: BoundedVec<u8, T::MaxTitleLength> =
					title.clone().try_into().map_err(|()| Error::<T>::TitleLenOverflow).unwrap();
				let bounded_author: BoundedVec<u8, T::MaxAuthorLength> =
					author.clone().try_into().map_err(|()| Error::<T>::AuthorLenOverflow).unwrap();
				let bounded_page: BoundedVec<u8, T::MaxPageLength> =
					page.clone().try_into().map_err(|()| Error::<T>::PageLenOverflow).unwrap();

				let mut pages = Vec::new();
				pages.push(bounded_page);
				let bounded_pages: BoundedVec<BoundedVec<u8, T::MaxPageLength>, T::MaxPageNum> =
					pages.try_into().map_err(|()| Error::<T>::PageLenOverflow).unwrap();

				let l = Letter {
					id: hash.clone(),
					title: bounded_title,
					author: bounded_author,
					price: balance.clone(),
					pages: bounded_pages,
				};

				let _ = <Pallet<T>>::mint_letter(acct.clone(), hash.clone(), l);
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		LetterInit(T::AccountId, T::Hash),
		PageWritten(T::AccountId, T::Hash),
		PriceSet(T::AccountId, T::Hash, T::Balance),
		Transferred(T::AccountId, T::AccountId, T::Hash),
		Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		NonceOverflow,
		NonExistentLetter,
		NonExistentPage,
		LetterCountOverflow,
		TitleLenOverflow,
		PageLenOverflow,
		AuthorLenOverflow,
		PageCountOverflow,
		LetterNotOwned,
	}

	// Dispatchable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as pallet::Config>::WeightInfo::init_letter(title.len() as u32, author.len() as u32))]
		pub fn init_letter(
			origin: OriginFor<T>,
			title: Vec<u8>,
			author: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_title: BoundedVec<u8, T::MaxTitleLength> =
				title.try_into().map_err(|()| Error::<T>::TitleLenOverflow)?;

			let bounded_author: BoundedVec<u8, T::MaxAuthorLength> =
				author.try_into().map_err(|()| Error::<T>::AuthorLenOverflow)?;

			let letter_id = Self::letter_id(&sender, bounded_title.clone(), bounded_author.clone());

			let pages: Vec<BoundedVec<u8, T::MaxPageLength>> = Vec::new();
			let bounded_pages: BoundedVec<BoundedVec<u8, T::MaxPageLength>, T::MaxPageNum> =
				pages.try_into().map_err(|()| Error::<T>::PageLenOverflow)?;

			let letter = Letter {
				id: letter_id,
				title: bounded_title,
				author: bounded_author,
				price: 0u8.into(),
				pages: bounded_pages,
			};

			Self::mint_letter(sender, letter_id, letter)?;
			Self::increment_nonce()?;

			Ok(())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::write_page(page.len() as u32))]
		pub fn write_page(
			origin: OriginFor<T>,
			letter_id: T::Hash,
			page: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if page.len() > T::MaxPageLength::get() as usize {
				return Err(Error::<T>::PageLenOverflow.into())
			}

			let (letter, _) = match Self::letter(letter_id) {
				Some((l, _)) => (l, ..),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};

			let page_count = letter.pages.len();
			if page_count == T::MaxPageNum::get() as usize {
				return Err(Error::<T>::PageCountOverflow.into())
			}

			Self::mint_page(sender.clone(), letter_id, page)?;

			Self::deposit_event(Event::PageWritten(sender, letter_id));

			Ok(())
		}

		// set_price
		#[pallet::weight(<T as pallet::Config>::WeightInfo::set_price())]
		pub fn set_price(
			origin: OriginFor<T>,
			letter_id: T::Hash,
			new_price: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Make sure the letter exists.
			ensure!(<Letters<T>>::contains_key(letter_id), "This letter does not exist");

			// Check that the letter has an owner (i.e. if it exists).
			let owner = Self::owner_of(letter_id).ok_or("No owner for this letter")?;
			// Make sure the owner matches the corresponding owner.
			ensure!(owner == sender, "You do not own this letter");

			// Set the Letter price.
			let (mut letter, reserve) = match Self::letter(letter_id) {
				Some((l, r)) => (l, r),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};
			letter.price = new_price;

			// Update new letter infomation to storage.
			<Letters<T>>::insert(letter_id, (letter, reserve));

			Self::deposit_event(Event::PriceSet(sender, letter_id, new_price));

			Ok(().into())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			letter_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Verify letter owner: must be the account invoking this transaction.
			let owner = Self::owner_of(letter_id).ok_or("No owner for this letter")?;
			ensure!(owner == sender, "You do not own this letter");

			// Transfer.
			Self::transfer_from(sender, to, letter_id)?;

			Ok(().into())
		}

		// buy_letter
		#[pallet::weight(<T as pallet::Config>::WeightInfo::buy_letter())]
		pub fn buy_letter(
			origin: OriginFor<T>,
			letter_id: T::Hash,
			ask_price: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Check if the letter exists.
			ensure!(<Letters<T>>::contains_key(letter_id), "This letter does not exist");

			// Check that the letter has an owner.
			let owner = Self::owner_of(letter_id).ok_or("No owner for this letter")?;

			// Check that account buying the letter doesn't already own it.
			ensure!(owner != sender, "You can't buy your own letter");

			// Get the price of the letter
			let (mut letter, reserve) = match Self::letter(letter_id) {
				Some((l, r)) => (l, r),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};
			let letter_price = letter.price;

			// Check if the letter is for sale.
			ensure!(!letter_price.is_zero(), "This letter is not for sale!");
			// Check that the letter's current price is within buyers budget.
			ensure!(letter_price <= ask_price, "This letter is out of your budget!");

			// Update Balances using the Currency trait.
			<pallet_balances::Pallet<T> as Currency<_>>::transfer(
				&sender,
				&owner,
				letter_price,
				ExistenceRequirement::KeepAlive,
			)?;

			// Transfer ownership of letter
			Self::transfer_from(owner.clone(), sender.clone(), letter_id).expect(
				"`owner` is shown to own the letter; \
    				`owner` must have greater than 0 letters, so transfer cannot cause underflow; \
    				`all_letter_count` shares the same type as `owned_letter_count` \
    				and minting ensure there won't ever be more than `max()` letter, \
    				which means transfer cannot cause an overflow;",
			);

			// Set the price of the letter to the new price it was sold at.
			letter.price = ask_price.into();
			<Letters<T>>::insert(letter_id, (letter, reserve));

			Self::deposit_event(Event::Bought(sender, owner, letter_id, letter_price));

			Ok(().into())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		// Helper to increment nonce
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
				*nonce = next;

				Ok(().into())
			})
		}

		// Helper to generate letter id
		fn letter_id(
			sender: &T::AccountId,
			title: BoundedVec<u8, T::MaxTitleLength>,
			author: BoundedVec<u8, T::MaxAuthorLength>,
		) -> T::Hash {
			let nonce = <Nonce<T>>::get();

			T::Hashing::hash_of(&(title, author, &sender, nonce))
		}

		// Helper to mint Letter
		fn mint_letter(
			to: T::AccountId,
			letter_id: T::Hash,
			new_letter: Letter<T>,
		) -> DispatchResult {
			ensure!(!<LetterOwner<T>>::contains_key(letter_id), "Letter already contains_key");

			// update owned letters count
			let owned_letter_count = Self::owned_letter_count(&to);
			let new_owned_letter_count = owned_letter_count
				.checked_add(1)
				.ok_or("Overflow adding a new letter to account balance")?;

			// update all letters count
			let all_letters_count = Self::all_letters_count();
			let new_all_letters_count = all_letters_count
				.checked_add(1)
				.ok_or("Overflow adding a new letter to total supply")?;

			// reserve page deposit
			let reserve = T::LetterDepositBase::get() +
				T::LetterDepositFactor::get() * (new_letter.title.len() as u32).into() +
				T::LetterDepositFactor::get() * (new_letter.author.len() as u32).into();
			T::Currency::reserve(&to, reserve)?;

			// update storage with new letter
			<Letters<T>>::insert(letter_id, (new_letter, reserve));
			<LetterOwner<T>>::insert(letter_id, Some(&to));

			// write letter counting information to storage
			<OwnedLettersArray<T>>::insert((to.clone(), new_owned_letter_count), letter_id);
			<OwnedLettersCount<T>>::insert(&to, new_owned_letter_count);
			<OwnedLettersIndex<T>>::insert(letter_id, new_owned_letter_count);
			<AllLettersArray<T>>::insert(new_all_letters_count, letter_id);
			<AllLettersCount<T>>::put(new_all_letters_count);
			<AllLettersIndex<T>>::insert(letter_id, new_all_letters_count);

			// write LetterInit event
			Self::deposit_event(Event::LetterInit(to, letter_id));

			Ok(())
		}

		// Helper to mint page
		fn mint_page(sender: T::AccountId, letter_id: T::Hash, page: Vec<u8>) -> DispatchResult {
			// check letter exists
			ensure!(<LetterOwner<T>>::contains_key(letter_id), "Letter non-existent");

			// check sender owns the letter
			let letter_owner = Self::owner_of(letter_id);
			if letter_owner != Some(sender.clone()) {
				return Err(Error::<T>::LetterNotOwned.into())
			}

			let (mut letter, mut reserve) = match Self::letter(letter_id) {
				Some((l, r)) => (l, r),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};

			let bounded_page: BoundedVec<u8, T::MaxPageLength> =
				page.try_into().map_err(|()| Error::<T>::PageLenOverflow)?;
			match letter.pages.try_push(bounded_page.clone()) {
				Ok(_) => (),
				Err(_) => return Err(Error::<T>::PageCountOverflow.into()),
			};

			T::Currency::unreserve(&sender, reserve);
			reserve += T::PageDepositBase::get() +
				T::PageDepositFactor::get() * (bounded_page.len() as u32).into();
			T::Currency::reserve(&sender, reserve)?;

			<Letters<T>>::insert(letter_id, (letter, reserve));

			Ok(())
		}

		// Helper to handle transferring a Letter from one account to another.
		fn transfer_from(
			from: T::AccountId,
			to: T::AccountId,
			letter_id: T::Hash,
		) -> DispatchResult {
			// verify rightful owner
			let owner = Self::owner_of(letter_id).ok_or("No owner for this letter")?;
			ensure!(owner == from, "account does not own this letter");

			let (_, reserve) = match Self::letter(letter_id) {
				Some((_, r)) => (.., r),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};

			T::Currency::unreserve(&from, reserve);
			T::Currency::reserve(&to, reserve)?;

			// count of letters owned by address to send from
			let owned_letter_count_from = Self::owned_letter_count(&from);

			// increment the amount of owned letters
			let new_owned_letter_count_from = owned_letter_count_from
				.checked_sub(1)
				.ok_or("Transfer causes underflow of 'from' letter balance")?;

			// count of letters owned by address to send to
			let owned_letter_count_to = Self::owned_letter_count(&to);

			// increment the amount of owned letters
			let new_owned_letter_count_to = owned_letter_count_to
				.checked_add(1)
				.ok_or("Transfer causes overflow of 'to' letter balance")?;

			// get current letter index
			let letter_index = <OwnedLettersIndex<T>>::get(letter_id);

			// update storage items that require updated index.
			if letter_index != new_owned_letter_count_from {
				let last_letter_id =
					<OwnedLettersArray<T>>::get((from.clone(), new_owned_letter_count_from));
				<OwnedLettersArray<T>>::insert((from.clone(), letter_index), last_letter_id);
				<OwnedLettersIndex<T>>::insert(last_letter_id, letter_index);
			}

			// write newletter ownership to storage items
			<LetterOwner<T>>::insert(&letter_id, Some(&to));
			<OwnedLettersIndex<T>>::insert(letter_id, owned_letter_count_to);
			<OwnedLettersArray<T>>::remove((from.clone(), new_owned_letter_count_from));
			<OwnedLettersArray<T>>::insert((to.clone(), new_owned_letter_count_to), letter_id);
			<OwnedLettersCount<T>>::insert(&from, new_owned_letter_count_from);
			<OwnedLettersCount<T>>::insert(&to, new_owned_letter_count_to);

			Self::deposit_event(Event::Transferred(from, to, letter_id));

			Ok(())
		}

		pub fn read_page(
			letter_id: T::Hash,
			page_index: usize,
		) -> sp_std::result::Result<BoundedVec<u8, T::MaxPageLength>, DispatchError> {
			let (letter, _) = match Self::letter(letter_id) {
				Some((l, _)) => (l, ..),
				None => return Err(Error::<T>::NonExistentLetter.into()),
			};

			// check page exists
			if letter.pages.len() == 0 || letter.pages.len() <= page_index {
				return Err(Error::<T>::NonExistentPage.into())
			}

			let page = letter.pages[page_index].clone();
			Ok(page)
		}
	}
}
