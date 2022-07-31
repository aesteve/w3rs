use std::fmt;
use std::fmt::{Display, Formatter};
use crate::blocks::gameblock::GameBlock;
use crate::game::{Game, GameOutcome};

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let outcome = self.outcome();
        writeln!(f, "Warcraft 3 Reforged game. {:?}", self.game_type())?;
        writeln!(f, "\tMap: {}", self.map.name)?;
        for (team, players) in self.players_by_team() {
            let team_won = outcome == GameOutcome::Winner(team);
            write!(f, "\tTeam {:?}:", team + 1)?;
            write!(f, " [ ")?;
            for player in players {
                write!(f, "{} ", player)?;
            }
            write!(f, "]")?;
            if team_won {
                write!(f, " ✅")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for GameBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameBlock::PlayerChatMsg(msg) => writeln!(f, "Player {}: {}", msg.player_id, msg.text)?,
            GameBlock::Leave(left) => writeln!(
                f,
                "Player {} left {:?}|{:?}",
                left.player_id, left.reason, left.result
            )?,
            GameBlock::TimeSlot(ts_block) => {
                if let Some(cmd) = &ts_block.command {
                    writeln!(f, "Player {}:", cmd.player)?;
                    for action in &cmd.actions {
                        writeln!(f, "\t{:?}", action)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
