# System Architecture

## Overview

Microcosm is a decentralized territory-based economy built on Solana. It uses a dual-token model (MCC + MCD) with on-chain mining, autonomous buyback, and monthly reincarnation cycles.

```
                    +-------------------+
                    |   Oracle Service   |
                    |  (Price Updates)   |
                    +---------+---------+
                              |
                    +---------v---------+
                    |   Reincarnation    |
                    |     Program        |
                    |                    |
                    |  - Mining          |
                    |  - Buyback         |
                    |  - Monthly Cycle   |
                    +---------+---------+
                              |
              +---------------+---------------+
              |               |               |
    +---------v---+   +-------v-----+   +-----v-------+
    |  MCC Token  |   |  MCD Token  |   |   Auction   |
    |  Program    |   |  Program    |   |   Program   |
    +-------------+   +-------------+   +-------------+
```

## Program Descriptions

### Reincarnation Program
The core economic engine. Handles three primary operations:

1. **Mining** (`execute_mining`): Users pay stablecoins to mine MCC. Simultaneously produces companion yield tokens that flow to team, magistrate, and territory vaults.

2. **Buyback** (`execute_buyback`): Users sell MCC back at a 5% premium over base price. Supports both USDC and USDT stablecoins.

3. **Monthly Cycle** (`execute_monthly_cycle`): On the 1st of each month, accumulated MCC and MCD in reincarnation vaults return to the Genesis pool.

### MCC Token Program
Manages MCC token configuration and legacy mining operations. The program was the original mining entry point before migration to the Reincarnation program.

### MCD Token Program
Manages MCD token operations including:
- Station vault initialization
- MCD distribution to territory treasuries
- Balance tracking and reporting

### Auction Program
Handles territory magistrate elections through on-chain auctions:
- Auction creation with configurable parameters
- Bid placement with MCC
- Automatic settlement and magistrate assignment

### Fragment Program
Manages MCC fragment collection:
- Fragment minting and distribution
- Fragment-to-MCC redemption
- Collection tracking

### Lending Program
Collateralized lending protocol:
- Deposit collateral (MCC/MCD)
- Borrow against collateral
- Liquidation mechanics
- Interest rate management

### Level Verifier Program
On-chain verification of user levels:
- Level proof generation
- Cross-program level checks
- Level-gated access control

### Territory NFT Program
Territory ownership representation:
- NFT minting for territory ownership
- Transfer and delegation
- Metadata management

## Token Flow

```
User pays USDC/USDT
        |
        v
  [Reincarnation Program]
        |
        +---> 50% MCC --> User wallet
        +---> 10% MCC --> Team vault
        +---> 10% MCC --> Magistrate wallets (4%/3%/2%/1%)
        +---> 30% MCD --> Territory treasury
        |
  USDC/USDT stored in vault
        |
        v
  [Buyback] User sells MCC --> receives USDC/USDT from vault
        |
        v
  [Monthly Cycle] MCC + MCD vaults --> Genesis pool
```

## Security Model

### Authority Control
- Single authority address controls all administrative functions
- Authority verification via Solana's native `Signer` constraint
- PDA (Program Derived Address) for pool state — deterministic, no private key

### Token Safety
- MCC mint authority permanently revoked
- All LP tokens burned (permanent liquidity)
- StreamFlow vesting is non-cancelable
- Checked arithmetic throughout (no overflow/underflow)

### Access Control Hierarchy
```
Authority (admin)
  |
  +-- Update price (oracle integration)
  +-- Pause/unpause pool
  +-- Withdraw funds
  +-- Initialize vaults
  |
Users (permissionless)
  |
  +-- Execute mining
  +-- Execute buyback
```
