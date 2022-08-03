use crate as pallet_letters;
use frame_support::{parameter_types, traits::Randomness};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const LETTER_DEPOSIT_BASE: u32 = 50;
pub const LETTER_DEPOSIT_FACTOR: u32 = 5;
pub const PAGE_DEPOSIT_BASE: u32 = 10;
pub const PAGE_DEPOSIT_FACTOR: u32 = 1;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Letters: pallet_letters::{Pallet, Call, Storage, Event<T>},
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
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxTitleLength: u32 = 64;
	pub const MaxAuthorLength: u32 = 64;
	pub const MaxPageLength: u32 = 8192;
	pub const MaxPageNum: u32 = 64;
	pub const LetterDepositBase: u32 = LETTER_DEPOSIT_BASE;
	pub const LetterDepositFactor: u32 = LETTER_DEPOSIT_FACTOR;
	pub const PageDepositBase: u32 = PAGE_DEPOSIT_BASE;
	pub const PageDepositFactor: u32 = PAGE_DEPOSIT_FACTOR;
}

impl pallet_letters::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type MaxAuthorLength = MaxAuthorLength;
	type MaxPageLength = MaxPageLength;
	type MaxPageNum = MaxPageNum;
	type MaxTitleLength = MaxTitleLength;
	type WeightInfo = ();
	type LetterDepositBase = LetterDepositBase;
	type LetterDepositFactor = LetterDepositFactor;
	type PageDepositBase = PageDepositBase;
	type PageDepositFactor = PageDepositFactor;
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

impl Randomness<H256, u32> for MockRandom {
	fn random(_subject: &[u8]) -> (H256, u32) {
		(MockRandom::get(), 0)
	}
}

/// Balance of an account.
pub type Balance = u128;

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}
