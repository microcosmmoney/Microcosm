# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in any Microcosm smart contract, please report it responsibly.

**Email:** security@microcosm.money

Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a detailed response within 7 days.

## Scope

This security policy covers:
- All Solana smart contracts in this repository
- On-chain program logic and state management
- Token operations (minting, transfers, burns)
- PDA derivation and access control

## Security Measures

### Key Management
- All private keys are managed via secure infrastructure (GCP Secret Manager)
- No private keys or secrets are stored in this repository
- Authority keys use hardware security modules where applicable

### Contract Security
- All admin operations require authority signature verification
- PDA seeds are deterministic and documented
- Token operations use checked arithmetic (overflow protection)
- Critical operations emit on-chain events for auditability

### Token Security
- **MCC Mint Authority**: Revoked (set to `None`) - no new MCC can ever be minted
- **MCD Mint Authority**: Retained for internal point system operations
- **Freeze Authority**: Revoked for both MCC and MCD
- **LP Tokens**: Fully burned - liquidity pool is permanent and irrevocable

### Deployed Programs
| Program | Address |
|---------|---------|
| MCC Token | `mCCUkDxoDfnVjTQjQHWkVi1PWBU2jxfQGJUhhDbeq5x` |
| MCD Token | `McDVieuEFv5ucnM3C7p8wsT6LY8BAipKDrTG9HjAu3R` |
| Reincarnation | `REn8oKyydvjRsistZ2cVi6tksPubvR3bEuLdVTyGknb` |
| Auction | `6JRmShodszZca3ez7GfDkmsRSzTi5ELwscWCDgjsteJx` |

## Audit Status

These contracts have been internally reviewed. Community audit contributions are welcome.

## Bug Bounty

We are evaluating a formal bug bounty program. In the meantime, responsible disclosures will be rewarded at our discretion.
