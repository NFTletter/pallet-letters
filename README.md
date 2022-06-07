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
The number of characters written into each letter determines the fees to be paid for the extrinsic which will write it into storage.

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

These constants need to be set on `src/runtime.rs` of the chain:
```rust
parameter_types! {
    ...
    pub const MaxTitleLength: u32 = 64;
    pub const MaxAuthorLength: u32 = 64;
    pub const MaxPageLength: u32 = 8192;
    pub const MaxPageNum: u32 = 64;
}
```
