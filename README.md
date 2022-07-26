# 📜 Substrate Letters Pallet ✍️

---
Inspired by [Substrate Ktties](https://substrate.dev/substrate-how-to-guides/docs/tutorials/Kitties/overview/) and [NFT Letter](https://nftletter.github.io/), this is a Substrate Pallet meant for text-based on-chain collectibles, such as **poems**, **novels** and **manifestos**.

According to the [Storage Best Practices](https://substrate.dev/docs/en/knowledgebase/runtime/storage#best-practices) from the *Substrate Developer Hub*:

> **What to Store**
> 
> Remember, the fundamental principle of blockchain runtime storage is to minimize its use. Only consensus-critical data should be stored in your runtime. ...

So you might wonder: 
*"Why would I want to store collectible text on the runtime storage? Why not use the traditional off-chain approach, like IPFS?"*

Letters Pallet is more of a token economics experiment and a learning experience rather than something to be used in production.
We want to explore the possibility of writing text on-chain, as well as the economic conditions necessary to make it sustainable.

A letter is represented by:
```rust
pub struct Letter<T: Config> {
    pub id: T::Hash,
    pub title: BoundedVec<u8, T::MaxTitleLength>,
    pub author: BoundedVec<u8, T::MaxAuthorLength>,
    pub price: T::Balance,
    pages: BoundedVec<BoundedVec<u8, T::MaxPageLength>, T::MaxPageNum>,
}
```

Letter sizes are bound to upper limits, defined by runtime constants:
```rust
#[pallet::constant]
type MaxTitleLength: Get<u32>;

#[pallet::constant]
type MaxAuthorLength: Get<u32>;

#[pallet::constant]
type MaxPageLength: Get<u32>;

#[pallet::constant]
type MaxPageNum: Get<u32>;
```

Reserve deposits are also defined by runtime constants:
```rust
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
```

These constants are set on the configuration of the pallet at `src/runtime.rs` of the chain:
```rust
impl pallet_letters::Config for Runtime {
    // ...
    type MaxAuthorLength = ConstU32<64>;
    type MaxPageLength = ConstU32<64>;
    type MaxPageNum = ConstU32<8192>;
    type MaxTitleLength = ConstU32<64>;
    type LetterDepositBase = ConstU128<50>;
    type LetterDepositFactor = ConstU128<5>;
    type PageDepositBase = ConstU128<10>;
    type PageDepositFactor = ConstU128<1>;
}
```

Therefore, the number of bytes written into the title, author and pages of each letter determines the reserve deposits when writing them into storage.

