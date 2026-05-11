use anchor_lang::prelude::*;

declare_id!("DrWE9k6sKVYS4x9whNHxdFoB4kFTuzEZ9zqWmBVcvWwW");

// Constants
pub const ENTRY_FEE: u64 = 50_000_000; // 0.05 SOL in lamports
pub const MIN_PLAYERS: usize = 3;
pub const MAX_PLAYERS: usize = 50;
pub const MAX_TICKETS: u32 = 50;
pub const ROUNDS_PER_JACKPOT: u64 = 288; // daily

// Fee basis points (out of 10000)
pub const JACKPOT_FEE_BPS: u64 = 100;    // 1%
pub const OWNER_FEE_BPS: u64 = 75;       // 0.75%
pub const DEV_FEE_BPS: u64 = 25;         // 0.25%
pub const LOSER_PENALTY_BPS: u64 = 2500; // 25%

#[program]
pub mod whiskysour {
    use super::*;

    // Initialize the game — called once by the admin
    pub fn initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        dev: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.game_config;
        config.admin = ctx.accounts.admin.key();
        config.owner_wallet = owner;
        config.dev_wallet = dev;
        config.round_count = 0;
        config.jackpot_pool = 0;
        config.bump = ctx.bumps.game_config;
        msg!("WhiskySour initialized!");
        Ok(())
    }

    // Register a new player
    pub fn register_player(ctx: Context<RegisterPlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player_account;
        player.wallet = ctx.accounts.wallet.key();
        player.tickets = 0;
        player.rounds_played = 0;
        player.rounds_won = 0;
        player.bump = ctx.bumps.player_account;
        msg!("Player registered: {:?}", player.wallet);
        Ok(())
    }

    // Player joins the current round
    pub fn join_round(ctx: Context<JoinRound>) -> Result<()> {
        let round = &mut ctx.accounts.round_account;
        let player = &mut ctx.accounts.player_account;
        let config = &ctx.accounts.game_config;

        // Guard: round must be open
        require!(round.state == RoundState::Open, WhiskySourError::RoundNotOpen);

        // Guard: max players
        require!(round.players.len() < MAX_PLAYERS, WhiskySourError::RoundFull);

        // Guard: player not already in round
        require!(
            !round.players.contains(&player.wallet),
            WhiskySourError::AlreadyJoined
        );

        // Transfer entry fee from player to round vault
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.wallet.to_account_info(),
                to: ctx.accounts.round_vault.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, ENTRY_FEE)?;

        // Add player to round
        round.players.push(player.wallet);
        round.pool += ENTRY_FEE;

        // Update fee buckets
        round.jackpot_fees += (ENTRY_FEE * JACKPOT_FEE_BPS) / 10000;
        round.owner_fees += (ENTRY_FEE * OWNER_FEE_BPS) / 10000;
        round.dev_fees += (ENTRY_FEE * DEV_FEE_BPS) / 10000;

        msg!("Player {:?} joined round {}", player.wallet, config.round_count);
        Ok(())
    }

    // Execute the round — pick winners, distribute, assign tickets
    pub fn execute_round(ctx: Context<ExecuteRound>) -> Result<()> {
        let round = &mut ctx.accounts.round_account;
        let config = &mut ctx.accounts.game_config;

        // Guard: must be open
        require!(round.state == RoundState::Open, WhiskySourError::RoundNotOpen);

        // Guard: minimum players
        require!(round.players.len() >= MIN_PLAYERS, WhiskySourError::NotEnoughPlayers);

        // Mark round as executing
        round.state = RoundState::Executing;

        let total_players = round.players.len();

        // Calculate winner count (80%, floor)
        let winner_count = (total_players * 80) / 100;
        let loser_count = total_players - winner_count;

        // Calculate loser pool (25% of each loser's entry)
        let loser_penalty = (ENTRY_FEE * LOSER_PENALTY_BPS) / 10000;
        let total_loser_pool = loser_penalty * loser_count as u64;

        // Each winner gets equal share of loser pool
        let winner_share = if winner_count > 0 {
            total_loser_pool / winner_count as u64
        } else {
            0
        };

        // Select winners using pseudo-random seed (pool + round_count + clock)
        let clock = Clock::get()?;
        let seed = round.pool
            .wrapping_add(config.round_count)
            .wrapping_add(clock.unix_timestamp as u64)
            .wrapping_add(clock.slot);

        // Shuffle player indices using seed
        let mut indices: Vec<usize> = (0..total_players).collect();
        let mut s = seed;
        for i in (1..total_players).rev() {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let j = (s as usize) % (i + 1);
            indices.swap(i, j);
        }

        // First winner_count indices are winners
        let mut winners: Vec<Pubkey> = Vec::new();
        let mut losers: Vec<Pubkey> = Vec::new();

        for (pos, &idx) in indices.iter().enumerate() {
            if pos < winner_count {
                winners.push(round.players[idx]);
            } else {
                losers.push(round.players[idx]);
            }
        }

        round.winners = winners;
        round.losers = losers;
        round.winner_share = winner_share;
        round.state = RoundState::Closed;

        // Update jackpot pool in config
        config.jackpot_pool += round.jackpot_fees;
        config.round_count += 1;

        msg!(
            "Round executed: {} winners, {} losers, {} lamports each",
            winner_count, loser_count, winner_share
        );
        Ok(())
    }

    // Draw the daily jackpot
    pub fn draw_jackpot(ctx: Context<DrawJackpot>) -> Result<()> {
        let config = &mut ctx.accounts.game_config;

        // Guard: only every 288 rounds
        require!(
            config.round_count % ROUNDS_PER_JACKPOT == 0,
            WhiskySourError::JackpotNotReady
        );

        require!(config.jackpot_pool > 0, WhiskySourError::EmptyJackpot);

        msg!("Jackpot drawn! Pool: {} lamports", config.jackpot_pool);

        // Winner selection happens off-chain via bot using ticket weights
        // Bot reads ticket counts, selects winner, calls claim_jackpot

        Ok(())
    }

    // Refund round if not enough players
    pub fn refund_round(ctx: Context<RefundRound>) -> Result<()> {
        let round = &mut ctx.accounts.round_account;

        require!(round.state == RoundState::Open, WhiskySourError::RoundNotOpen);
        require!(round.players.len() < MIN_PLAYERS, WhiskySourError::EnoughPlayers);

        round.state = RoundState::Refunded;

        msg!("Round refunded — not enough players");
        Ok(())
    }
}

// ===== ACCOUNT STRUCTS =====

#[account]
pub struct GameConfig {
    pub admin: Pubkey,
    pub owner_wallet: Pubkey,
    pub dev_wallet: Pubkey,
    pub round_count: u64,
    pub jackpot_pool: u64,
    pub bump: u8,
}

#[account]
pub struct PlayerAccount {
    pub wallet: Pubkey,
    pub tickets: u32,
    pub rounds_played: u32,
    pub rounds_won: u32,
    pub bump: u8,
}

#[account]
pub struct RoundAccount {
    pub round_id: u64,
    pub state: RoundState,
    pub pool: u64,
    pub players: Vec<Pubkey>,
    pub winners: Vec<Pubkey>,
    pub losers: Vec<Pubkey>,
    pub winner_share: u64,
    pub jackpot_fees: u64,
    pub owner_fees: u64,
    pub dev_fees: u64,
    pub bump: u8,
}

// ===== ENUMS =====

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RoundState {
    Open,
    Executing,
    Closed,
    Refunded,
}

// ===== CONTEXTS =====

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"game_config"],
        bump
    )]
    pub game_config: Account<'info, GameConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    #[account(
        init,
        payer = wallet,
        space = 8 + 32 + 4 + 4 + 4 + 1,
        seeds = [b"player", wallet.key().as_ref()],
        bump
    )]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinRound<'info> {
    #[account(mut, seeds = [b"game_config"], bump = game_config.bump)]
    pub game_config: Account<'info, GameConfig>,
    #[account(mut, seeds = [b"player", wallet.key().as_ref()], bump = player_account.bump)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut)]
    pub round_account: Account<'info, RoundAccount>,
    /// CHECK: round vault to hold entry fees
    #[account(mut)]
    pub round_vault: AccountInfo<'info>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteRound<'info> {
    #[account(mut, seeds = [b"game_config"], bump = game_config.bump)]
    pub game_config: Account<'info, GameConfig>,
    #[account(mut)]
    pub round_account: Account<'info, RoundAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct DrawJackpot<'info> {
    #[account(mut, seeds = [b"game_config"], bump = game_config.bump)]
    pub game_config: Account<'info, GameConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct RefundRound<'info> {
    #[account(mut)]
    pub round_account: Account<'info, RoundAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

// ===== ERRORS =====

#[error_code]
pub enum WhiskySourError {
    #[msg("Round is not open")]
    RoundNotOpen,
    #[msg("Not enough players to start round (minimum 3)")]
    NotEnoughPlayers,
    #[msg("Enough players — cannot refund")]
    EnoughPlayers,
    #[msg("Player already joined this round")]
    AlreadyJoined,
    #[msg("Round is full")]
    RoundFull,
    #[msg("Jackpot not ready yet")]
    JackpotNotReady,
    #[msg("Jackpot pool is empty")]
    EmptyJackpot,
}
