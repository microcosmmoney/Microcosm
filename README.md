# Microcosm

Open-source Solana smart contracts powering the Microcosm ecosystem — a decentralized territory-based economy with dual-token mechanics, on-chain mining, and autonomous market cycles.

## Architecture

Microcosm consists of 8 on-chain programs:

| Program | ID | Description |
|---------|-----|-------------|
| **MCC Token** | `mCCUkDxoDfnVjTQjQHWkVi1PWBU2jxfQGJUhhDbeq5x` | Microcosm Coin — primary value token |
| **MCD Token** | `McDVieuEFv5ucnM3C7p8wsT6LY8BAipKDrTG9HjAu3R` | Microcosm Dollar — internal utility point |
| **Reincarnation** | `REn8oKyydvjRsistZ2cVi6tksPubvR3bEuLdVTyGknb` | Core engine: mining, buyback, monthly cycle |
| **Auction** | `6JRmShodszZca3ez7GfDkmsRSzTi5ELwscWCDgjsteJx` | Territory magistrate auction system |
| **Fragment** | — | MCC fragment collection and redemption |
| **Lending** | — | Collateralized lending protocol |
| **Level Verifier** | — | On-chain user level verification |
| **Territory NFT** | — | Territory ownership NFTs |

## Tokens

### MCC (Microcosm Coin)
| Property | Value |
|----------|-------|
| Standard | SPL Token + Metaplex Metadata |
| Mint | `MCCpDtigJLYnfGe1fW5xrSA8AXo6AeAj8ECE7wVqP5e` |
| Total Supply | 1,000,000,000 (1 billion) |
| Decimals | 9 |
| Mint Authority | **Revoked** (`None`) |
| Freeze Authority | **Revoked** (`None`) |
| Metadata | **Immutable** (`isMutable = false`) |

### MCD (Microcosm Dollar)
| Property | Value |
|----------|-------|
| Standard | SPL Token + Metaplex Metadata |
| Mint | `MCDXTiLK6idQSycdb8QkwKYVP8HEbfe4JbJZjC7Fkty` |
| Total Supply | 10,000,000,000 (10 billion) |
| Decimals | 9 |
| Mint Authority | Retained (internal point system) |
| Freeze Authority | **Revoked** (`None`) |

## Key Addresses

| Role | Address |
|------|---------|
| Authority | `MCWe3gEYqTjRCShEkhqbMXhcR4xP2dz29gurSpLbk4A` |
| Genesis Wallet | `3Cusf4eTVQKFnhdBpHtJb2FHpyRbGLm2YbGSwq3Yjr1N` |
| Team Wallet | `6hSsFEXmVqyPPVE2cG71Ai9rQvJeBN3fALvK2BZGSZrZ` |
| Raydium LP Pool | `HgK6MkCSiuEk7kpDLhdfpKm8i7STuxxwQVBxAo4oshLa` |
| LP Mint (Burned) | `8yGaiU3G7uT9hEE5HfeUAPTBFmkN3M6Pn7ff4n57cg9y` |
| StreamFlow Lock | `AdPsRzsiLmYkY8nwi2She34ZD5zA2kb6dxmEzBvciBAb` |

## Core Mechanics

### Mining
Users mine MCC by paying 2x the oracle market price in stablecoins (USDC/USDT). Each mining operation also produces **Companion Yield** — additional tokens that flow into the ecosystem:

- **50%** MCC to the user (mining output)
- **10%** MCC to team vault (companion yield)
- **10%** MCC to magistrate wallets (companion yield, split 4%/3%/2%/1% across 4 tiers)
- **30%** MCD to territory treasury (companion yield)

Mining price halves with cumulative output: 0-100M = 1:1, 100-200M = 2:1, 200-300M = 4:1, etc.

### Buyback (Reincarnation)
Users can sell MCC back to the protocol at `base_price * 1.05` (5% premium over oracle price). The contract accepts both USDC and USDT, paying from the respective stablecoin vault.

### Monthly Cycle
On the 1st of each month (with minimum 25-day intervals), accumulated MCC and MCD in reincarnation vaults are returned to the Genesis pool, completing the economic cycle.

### Territory System
| Level | Name | Capacity |
|-------|------|----------|
| Station | Space Station | 1,000 users |
| Matrix | Matrix | 10 Stations |
| Sector | Sector | 10 Matrices |
| System | Star System | 10 Sectors |

Magistrates (territory administrators) are elected through on-chain auctions.

### Token Locking
998,000,000 MCC (99.8% of supply) is locked via StreamFlow with:
- 100-month linear release (~9.98M MCC/month)
- Non-cancelable (irreversible)
- Release end date: April 2034

### Liquidity
- Trading pair: MCC/USDT on Raydium CPMM
- **All LP tokens have been burned** — the liquidity pool is permanent and cannot be withdrawn

## Repository Structure

```
contracts/
  auction/           # Territory magistrate auction
  mcc-token/         # MCC Token program
  mcd-token/         # MCD Token program
  reincarnation/     # Core: mining + buyback + monthly cycle
  fragment/          # MCC fragment collection
  lending/           # Collateralized lending
  level-verifier/    # On-chain level verification
  territory-nft/     # Territory ownership NFTs
docs/
  architecture.md    # System architecture overview
  tokenomics.md      # Token economics deep dive
  contracts.md       # Smart contract API reference
```

## Building

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) v1.18+
- [Anchor](https://www.anchor-lang.com/docs/installation) v0.30.1

### Build
```bash
cd contracts/<program-name>
anchor build
```

### Test
```bash
anchor test
```

## Security

See [SECURITY.md](SECURITY.md) for our security policy and responsible disclosure guidelines.

**Key security properties:**
- MCC mint authority is permanently revoked
- All LP tokens are burned (irrevocable liquidity)
- 99.8% of supply is in non-cancelable StreamFlow vesting
- All admin operations require cryptographic authority verification

## Links

- **Website:** [microcosm.money](https://microcosm.money)
- **Twitter/X:** [@MicrocosmMoney](https://x.com/MicrocosmMoney)
- **Token (Solscan):** [MCCpDtig...](https://solscan.io/token/MCCpDtigJLYnfGe1fW5xrSA8AXo6AeAj8ECE7wVqP5e)
- **Raydium Pool:** [HgK6Mk...](https://raydium.io/swap/?inputMint=Es9vMFrzaCERmKfrsSMYHnKRqjh4oiJBBe4rDE6bwuZY&outputMint=MCCpDtigJLYnfGe1fW5xrSA8AXo6AeAj8ECE7wVqP5e)

## License

[MIT](LICENSE)
