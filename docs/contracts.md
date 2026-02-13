# Smart Contract Reference

## Program IDs

| Program | ID | Status |
|---------|-----|--------|
| MCC Token | `mCCUkDxoDfnVjTQjQHWkVi1PWBU2jxfQGJUhhDbeq5x` | Deployed (Mainnet) |
| MCD Token | `McDVieuEFv5ucnM3C7p8wsT6LY8BAipKDrTG9HjAu3R` | Deployed (Mainnet) |
| Reincarnation | `REn8oKyydvjRsistZ2cVi6tksPubvR3bEuLdVTyGknb` | Deployed (Mainnet) |
| Auction | `6JRmShodszZca3ez7GfDkmsRSzTi5ELwscWCDgjsteJx` | Deployed (Mainnet) |

## Token Mints

| Token | Mint Address | Decimals | Standard |
|-------|-------------|----------|----------|
| MCC | `MCCpDtigJLYnfGe1fW5xrSA8AXo6AeAj8ECE7wVqP5e` | 9 | SPL Token |
| MCD | `MCDXTiLK6idQSycdb8QkwKYVP8HEbfe4JbJZjC7Fkty` | 9 | SPL Token |
| USDC | `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v` | 6 | SPL Token |
| USDT | `Es9vMFrzaCERmKfrsSMYHnKRqjh4oiJBBe4rDE6bwuZY` | 6 | SPL Token |

## Key Accounts

| Account | Address | Description |
|---------|---------|-------------|
| Authority | `MCWe3gEYqTjRCShEkhqbMXhcR4xP2dz29gurSpLbk4A` | Program admin |
| Genesis Wallet | `3Cusf4eTVQKFnhdBpHtJb2FHpyRbGLm2YbGSwq3Yjr1N` | Token source pool |
| Team Wallet | `6hSsFEXmVqyPPVE2cG71Ai9rQvJeBN3fALvK2BZGSZrZ` | Team MCC vault |
| Reincarnation Pool PDA | Derived from `[b"reincarnation_pool"]` | Pool state |
| USDC Vault | ATA of Pool PDA for USDC mint | Stablecoin reserves |
| USDT Vault | ATA of Pool PDA for USDT mint | Stablecoin reserves |
| MCC Vault | ATA of Pool PDA for MCC mint | Buyback MCC accumulator |
| MCD Vault | ATA of Pool PDA for MCD mint | Cycle MCD accumulator |

## Reincarnation Program Instructions

### `execute_mining`
Executes a mining operation. User pays stablecoins, receives MCC, and companion yield is distributed.

**Accounts (14):**
| # | Account | Signer | Writable | Description |
|---|---------|--------|----------|-------------|
| 1 | user | Yes | Yes | Mining user |
| 2 | reincarnation_pool | No | Yes | Pool PDA |
| 3 | mcc_mint | No | No | MCC token mint |
| 4 | mcd_mint | No | No | MCD token mint |
| 5 | usdc_mint | No | No | Stablecoin mint (USDC/USDT) |
| 6 | user_usdc_account | No | Yes | User's stablecoin ATA |
| 7 | usdc_vault | No | Yes | Pool stablecoin vault |
| 8 | genesis_mcc_account | No | Yes | Genesis MCC source |
| 9 | user_mcc_account | No | Yes | User's MCC ATA |
| 10 | team_mcc_account | No | Yes | Team MCC vault |
| 11 | magistrate_accounts | No | Yes | Up to 4 magistrate wallets |
| 12 | station_mcd_vault | No | Yes | Territory MCD vault |
| 13 | token_program | No | No | SPL Token program |
| 14 | system_program | No | No | System program |

**Arguments:**
- `mcc_amount: u64` — Amount of MCC to mine (9 decimals)

### `execute_buyback`
User sells MCC back to the protocol at a premium price.

**Accounts (10):**
| # | Account | Signer | Writable | Description |
|---|---------|--------|----------|-------------|
| 1 | user | Yes | Yes | Selling user |
| 2 | reincarnation_pool | No | Yes | Pool PDA |
| 3 | mcc_mint | No | No | MCC token mint |
| 4 | usdc_mint | No | No | Stablecoin mint |
| 5 | user_mcc_account | No | Yes | User's MCC ATA |
| 6 | user_usdc_account | No | Yes | User's stablecoin ATA |
| 7 | usdc_vault | No | Yes | Pool stablecoin vault |
| 8 | mcc_vault | No | Yes | Pool MCC vault |
| 9 | mcc_token_program | No | No | Token program for MCC |
| 10 | usdc_token_program | No | No | Token program for stablecoin |

**Arguments:**
- `mcc_amount: u64` — Amount of MCC to sell (9 decimals)

**Price Calculation:**
```
buyback_price = base_price * (1 + premium_bps / 10000)
usdc_amount = mcc_amount * buyback_price / 10^9
```

### `execute_monthly_cycle`
Returns accumulated MCC and MCD from vaults to Genesis pool.

**Constraints:**
- Can only execute on the 1st of each month
- Minimum 25-day interval between cycles

### `update_price`
Admin updates the oracle base price.

**Arguments:**
- `new_base_price: u64` — New price in stablecoin units (6 decimals)

### `pause_pool` / `unpause_pool`
Admin pauses or resumes all pool operations.

### `withdraw_usdc`
Admin withdraws stablecoins from the vault.

**Arguments:**
- `amount: u64` — Amount to withdraw (6 decimals)

## ATA (Associated Token Account) Calculation

All token accounts use the standard SPL Token ATA derivation:

```
ATA = PDA([owner_pubkey, TOKEN_PROGRAM_ID, mint_pubkey])
```

**Important:** MCC and MCD use `TOKEN_PROGRAM_ID` (not Token-2022). Using the wrong program ID will produce incorrect ATA addresses.

## Building Programs

```bash
# Prerequisites: Rust, Solana CLI v1.18+, Anchor v0.30.1

# Build a specific program
cd contracts/<program-name>
anchor build

# The compiled .so file will be in target/deploy/
```

## PDA Seeds Reference

| PDA | Seeds | Program |
|-----|-------|---------|
| Reincarnation Pool | `[b"reincarnation_pool"]` | Reincarnation |
| Mining Config | `[b"mining_config"]` | MCC Token |
| MCD Config | `[b"mcd_config"]` | MCD Token |
| Station Vault | `[b"station_vault", station_id]` | MCD Token |
| Auction | `[b"auction", auction_id]` | Auction |
