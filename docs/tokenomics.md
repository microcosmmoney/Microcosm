# Tokenomics

## Dual-Token Model

Microcosm operates with two complementary tokens:

### MCC (Microcosm Coin)
- **Role**: Primary value token, tradeable on DEX
- **Supply**: 1,000,000,000 (1 billion, fixed)
- **Decimals**: 9
- **Mint Authority**: Permanently revoked — no new MCC can ever be created
- **Distribution**: 99.8% locked in StreamFlow vesting (100-month linear release)

### MCD (Microcosm Dollar)
- **Role**: Internal utility point for territory operations
- **Supply**: 10,000,000,000 (10 billion)
- **Decimals**: 9
- **Mint Authority**: Retained for operational needs
- **Usage**: Territory treasury rewards, magistrate operations

## Mining Economics

### Pricing Formula
```
Mining Price = Oracle Market Price * 2
```

Users pay twice the current market price in stablecoins (USDC or USDT) and receive 100% of the mined MCC.

### Companion Yield
Each mining operation simultaneously produces companion yield — additional tokens that automatically flow into the ecosystem:

| Recipient | Share | Token | Description |
|-----------|-------|-------|-------------|
| User | 50% | MCC | Direct mining output |
| Team | 10% | MCC | Ecosystem development fund |
| Magistrate | 10% | MCC | Territory governance (4%/3%/2%/1% across 4 tiers) |
| Treasury | 30% | MCD | Territory operations fund |

With a developer assigned to the territory:
| Recipient | Share | Token |
|-----------|-------|-------|
| User | 50% | MCC |
| Team | 10% | MCC |
| Magistrate | 10% | MCC |
| Developer | 10% | MCD |
| Treasury | 20% | MCD |

### Halving Schedule

Mining cost doubles (output halves) at each 100M MCC cumulative output:

| Cumulative Output | Cost Ratio |
|-------------------|------------|
| 0 - 100M MCC | 1:1 |
| 100M - 200M MCC | 2:1 |
| 200M - 300M MCC | 4:1 |
| 300M - 400M MCC | 8:1 |
| ... | Doubles each 100M |

## Buyback Mechanism

The Reincarnation program offers automatic MCC buyback:

```
Buyback Price = Base Price * 1.05
```

- 5% premium over the oracle base price
- Accepts both USDC and USDT
- Daily limits enforced on-chain
- Minimum: 0.01 MCC, Maximum: 100,000 MCC per transaction

### Flow
1. User submits MCC to the contract
2. MCC goes to the reincarnation vault
3. User receives stablecoins from the vault at the buyback price
4. Accumulated MCC is returned to Genesis in the monthly cycle

## Monthly Reincarnation Cycle

On the 1st of each month (minimum 25-day interval):

1. All MCC accumulated in the reincarnation vault transfers to the Genesis pool
2. All MCD accumulated in the reincarnation vault transfers to the Genesis pool
3. Cycle statistics are updated on-chain
4. Event emitted for off-chain tracking

This creates a circular token flow:
```
Genesis Pool --> Mining --> Users --> Buyback --> Reincarnation Vault --> Genesis Pool
```

## Liquidity

### Raydium CPMM Pool
- **Pair**: MCC / USDT
- **Pool**: `HgK6MkCSiuEk7kpDLhdfpKm8i7STuxxwQVBxAo4oshLa`
- **LP Status**: All LP tokens permanently burned
- Liquidity can be added but never withdrawn

### StreamFlow Vesting
- **Locked**: 998,000,000 MCC (99.8% of total supply)
- **Schedule**: 100 months linear release (~9.98M MCC/month)
- **Cancelable**: No (irreversible)
- **End Date**: April 2034

## Supply Breakdown

| Allocation | Amount | Status |
|------------|--------|--------|
| StreamFlow Vesting | 998,000,000 MCC | Locked (100-month release) |
| Genesis Pool (Mining Buffer) | ~2,000,000 MCC | Active |
| Raydium Liquidity | ~1,000 MCC | Permanent (LP burned) |
