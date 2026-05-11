# 🥃 WhiskySour — Keep Winning.

> A real-time community game on Telegram where 80% of players win every round. Powered by Solana.

![WhiskySour Banner](https://img.shields.io/badge/Built%20on-Solana-9945FF?style=for-the-badge&logo=solana)
![Telegram](https://img.shields.io/badge/Play%20on-Telegram-26A5E4?style=for-the-badge&logo=telegram)
![Status](https://img.shields.io/badge/Status-Hackathon%20Build-F5C842?style=for-the-badge)

---

## What is WhiskySour?

WhiskySour is a Telegram-based community game that runs on Solana. Every 5 minutes, a round fires — players pay a small entry fee, 80% win, 20% lose. Losers don't lose everything — they lose only 25% of their entry and earn bonus jackpot tickets. The more you play, the better your jackpot odds.

**Keep Winning.**

---

## Why WhiskySour?

Most DeFi products are intimidating. WhiskySour makes on-chain value transfer feel like a game inside communities people already belong to. Telegram has 900M users — we meet them where they are.

The insight: **high win rate + small losses + ticket progression = habit-forming gameplay.**

Losing doesn't feel bad. You're building toward the jackpot.

---

## Core Mechanics

| Parameter | Value |
|---|---|
| Entry Fee | 0.05 SOL |
| Round Frequency | Every 5 minutes (288/day) |
| Win Rate | 80% of players |
| Loser Penalty | 25% of entry only |
| Minimum Players | 3 (full refund if under) |

### Fee Distribution
| Recipient | % |
|---|---|
| Jackpot Fund | 1% |
| Group Owner Wallet | 0.75% |
| Dev Fund | 0.25% |

### Ticket System
| Event | Tickets Earned |
|---|---|
| Win a round | +1 ticket |
| Lose a round | +2 tickets |
| Maximum tickets | 50 |

Tickets reset after each daily jackpot draw. More tickets = higher weighted odds of winning the jackpot.

---

## Game Loop

```
/play → Enter round → Win (80%) or Lose (20%)
  ↓                        ↓
+1 ticket             +2 tickets + keep 75% of entry
  ↓                        ↓
         Daily Jackpot Draw
         Weighted by ticket count
                ↓
         Tickets reset. Repeat.
```

---

## Telegram Commands

| Command | Description |
|---|---|
| `/start` | Register and connect wallet |
| `/play` | Enter the next round |
| `/status` | View balance, tickets, and odds |
| `/leaderboard` | Top players in your group |

---

## Tech Stack

| Layer | Technology |
|---|---|
| Blockchain | Solana (Devnet) |
| Smart Contract | Anchor Framework (Rust) |
| Telegram Bot | Rust (Teloxide) |
| Database | Supabase |
| Frontend | HTML/CSS (static) |
| Hosting | Vercel |

---

## Architecture

```
Telegram User
     ↓
Teloxide Bot (Rust)
     ↓
Anchor Program (Solana)
     ↓
┌─────────────────────────────┐
│  Round Account              │
│  Player Accounts            │
│  Jackpot Vault              │
│  Owner Wallet               │
└─────────────────────────────┘
     ↓
Supabase (tickets, history)
```

---

## Roadmap

### V1 — Hackathon Build (Current)
- [x] Website
- [x] Anchor program (round, player, jackpot accounts)
- [x] Telegram bot with /play, /start, /status
- [x] Devnet deployment
- [ ] Mainnet deployment
- [ ] More games More Communities 

### V2 — Drift Integration
The jackpot pool will be deposited into **Drift Protocol's lending market** between draws. The pool earns yield passively — making the jackpot self-growing even when no rounds are running.

```
Round fees → Jackpot Vault → Drift Lending → Yield accrues → Bigger jackpot
```

This transforms WhiskySour from a zero-sum game into a **positive-sum community game** — the house edge is replaced by DeFi yield.

### V3 — Growth
- Anti-sybil via wallet verification
- Group owner dashboard
- Multi-token support (USDC)
- Streak rewards

---

## Local Development

```bash
# Clone
git clone https://github.com/BrianGunner/Road_to_SolanaSpace_via_RustLand
cd Road_to_SolanaSpace_via_RustLand

# Build Anchor program
cd programs/whiskysour
anchor build

# Run tests
anchor test

# Run Telegram bot
cd bot
cargo run
```

---

## Team

Built by **BrianGunner** for the Colosseum Hackathon.

---

## Links

- Website: [whiskysour.live](https://whiskysour.live)
- Telegram: [@whisky_sour_bot](https://t.me/whisky_sour_bot)
- GitHub: [BrianGunner/Road_to_SolanaSpace_via_RustLand](https://github.com/BrianGunner/Road_to_SolanaSpace_via_RustLand)

---

*🥃 WhiskySour — Keep Winning.*
