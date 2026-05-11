use teloxide::{prelude::*, utils::command::BotCommands};
use dotenv::dotenv;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "WhiskySour Commands:")]
enum Command {
    #[command(description = "Register and start playing")]
    Start,
    #[command(description = "Join the current round (0.05 SOL)")]
    Play,
    #[command(description = "View your balance, tickets and odds")]
    Status,
    #[command(description = "Show all commands")]
    Help,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "🥃 *Welcome to WhiskySour\\!*\n\n\
                The community game where 80% of players win every round\\.\n\n\
                *How it works:*\n\
                • Pay 0\\.05 SOL to enter a round\n\
                • 80% of players win a share of the loser pool\n\
                • Losers keep 75% and earn \\+2 jackpot tickets\n\
                • Daily jackpot drawn every 288 rounds\n\n\
                Use /play to enter the next round\\.\n\
                Use /status to check your stats\\.\n\n\
                *Keep Winning\\. 🥃*",
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }

        Command::Play => {
            let username = msg
                .from()
                .and_then(|u| u.username.clone())
                .unwrap_or_else(|| "Player".to_string());

            bot.send_message(
                msg.chat.id,
                format!(
                    "🥃 *@{} is pouring a shot\\!*\n\n\
                    Round \\#143 — 5 players joined\n\
                    Pool: 0\\.25 SOL\n\
                    Next round in: 3:47\n\n\
                    ✅ You're in\\! Good luck\\.\n\n\
                    _Results will be announced when the round closes\\._",
                    escape_markdown(&username)
                ),
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }

        Command::Status => {
            let username = msg
                .from()
                .and_then(|u| u.username.clone())
                .unwrap_or_else(|| "Player".to_string());

            bot.send_message(
                msg.chat.id,
                format!(
                    "📊 *Stats for @{}*\n\n\
                    💰 Balance: 2\\.341 SOL\n\
                    🎮 Rounds Played: 47\n\
                    ✅ Rounds Won: 39\n\
                    📈 Win Rate: 83%\n\
                    💵 Profit: \\+0\\.16 SOL \\(\\+6\\.8%\\)\n\n\
                    🎟 Tickets: 32 / 50\n\
                    🏆 Jackpot Odds: \\~3\\.4%\n\n\
                    _Keep playing to earn more tickets\\!_",
                    escape_markdown(&username)
                ),
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }

        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    }

    Ok(())
}

fn escape_markdown(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if "._*[]()~`>#+-=|{}.!".contains(c) {
                vec!['\\', c]
            } else {
                vec![c]
            }
        })
        .collect()
}
