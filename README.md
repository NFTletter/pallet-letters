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
pub struct Letter<Hash, Balance> { 
    id: Hash,
    title: Vec<u8>,
    author: Vec<u8>,
    price: Balance,
    pages: Vec<Vec<u8>>,
}
```

Letter sizes are bound to upper limits, namely:
```rust
pub const MAX_TITLE_LEN: usize = 64;
pub const MAX_AUTHOR_LEN: usize = 64;
pub const MAX_NUM_PAGES: usize = 64;
pub const MAX_PAGE_LEN: usize = 8192;
```
Currently these bounds are set as constants, but the plan is to turn them into Genesis parameters.

The number of characters written into each letter determines the fees to be paid for the extrinsic which will write it into storage.

## Roadmap

- [x] Bootstrap
- [ ] [Minting Fees](https://substrate.dev/docs/en/knowledgebase/runtime/fees)
- [ ] Configurable upper bounds at Genesis
- [ ] [Tests](https://substrate.dev/docs/en/knowledgebase/runtime/tests)
- [ ] Front-end
