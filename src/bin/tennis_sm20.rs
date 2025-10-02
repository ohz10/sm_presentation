enum Player {
    Player1,
    Player2,
}

trait Point {
    fn point(&self, player: &Player) -> Game;
}

struct LoveLove;
struct LoveFifteen;
struct LoveThirty;
struct LoveForty;

struct FifteenLove;
struct FifteenFifteen;
struct FifteenThirty;
struct FifteenForty;

struct ThirtyLove;
struct ThirtyFifteen;
struct ThirtyThirty;
struct ThirtyForty;

struct FortyLove;
struct FortyFifteen;
struct FortyThirty;

struct Duece;
struct AdvantagePlayer1;
struct AdvantagePlayer2;

struct Player1Won;
struct Player2Won;
struct Error;

impl Point for LoveLove {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FifteenLove(self.into()),
            },
            Player::Player2 => Game {
                score: Score::LoveFifteen(self.into()),
            },
        }
    }
}

impl Point for LoveFifteen {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FifteenFifteen(self.into()),
            },
            Player::Player2 => Game {
                score: Score::LoveThirty(self.into()),
            },
        }
    }
}

impl Point for LoveThirty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FifteenThirty(self.into()),
            },
            Player::Player2 => Game {
                score: Score::LoveForty(self.into()),
            },
        }
    }
}

impl Point for LoveForty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FifteenForty(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Player2Won(self.into()),
            },
        }
    }
}

impl Point for FifteenLove {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::ThirtyLove(self.into()),
            },
            Player::Player2 => Game {
                score: Score::FifteenFifteen(self.into()),
            },
        }
    }
}

impl Point for FifteenFifteen {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::ThirtyFifteen(self.into()),
            },
            Player::Player2 => Game {
                score: Score::FifteenThirty(self.into()),
            },
        }
    }
}

impl Point for FifteenThirty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::ThirtyThirty(self.into()),
            },
            Player::Player2 => Game {
                score: Score::FifteenForty(self.into()),
            },
        }
    }
}

impl Point for FifteenForty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::ThirtyForty(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Player2Won(self.into()),
            },
        }
    }
}

impl Point for ThirtyLove {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FortyLove(self.into()),
            },
            Player::Player2 => Game {
                score: Score::ThirtyFifteen(self.into()),
            },
        }
    }
}

impl Point for ThirtyFifteen {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FortyFifteen(self.into()),
            },
            Player::Player2 => Game {
                score: Score::ThirtyThirty(self.into()),
            },
        }
    }
}

impl Point for ThirtyThirty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::FortyThirty(self.into()),
            },
            Player::Player2 => Game {
                score: Score::ThirtyForty(self.into()),
            },
        }
    }
}

impl Point for ThirtyForty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Duece(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Player2Won(self.into()),
            },
        }
    }
}

impl Point for FortyLove {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Player1Won(self.into()),
            },
            Player::Player2 => Game {
                score: Score::FortyFifteen(self.into()),
            },
        }
    }
}

impl Point for FortyFifteen {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Player1Won(self.into()),
            },
            Player::Player2 => Game {
                score: Score::FortyThirty(self.into()),
            },
        }
    }
}

impl Point for FortyThirty {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Player1Won(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Duece(self.into()),
            },
        }
    }
}

impl Point for Duece {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::AdvantagePlayer1(self.into()),
            },
            Player::Player2 => Game {
                score: Score::AdvantagePlayer2(self.into()),
            },
        }
    }
}

impl Point for AdvantagePlayer1 {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Player1Won(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Duece(self.into()),
            },
        }
    }
}

impl Point for AdvantagePlayer2 {
    fn point(&self, player: &Player) -> Game {
        match player {
            Player::Player1 => Game {
                score: Score::Duece(self.into()),
            },
            Player::Player2 => Game {
                score: Score::Player2Won(self.into()),
            },
        }
    }
}

impl Point for Player1Won {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }
}

impl Point for Player2Won {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }
}

impl Point for Error {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }
}

impl From<&LoveLove> for FifteenLove {
    fn from(_: &LoveLove) -> FifteenLove {
        FifteenLove
    }
}

impl From<&LoveLove> for LoveFifteen {
    fn from(_: &LoveLove) -> LoveFifteen {
        LoveFifteen
    }
}

impl From<&FifteenLove> for FifteenFifteen {
    fn from(_: &FifteenLove) -> FifteenFifteen {
        FifteenFifteen
    }
}

impl From<&LoveFifteen> for FifteenFifteen {
    fn from(_: &LoveFifteen) -> FifteenFifteen {
        FifteenFifteen
    }
}

impl From<&FifteenLove> for ThirtyLove {
    fn from(_: &FifteenLove) -> ThirtyLove {
        ThirtyLove
    }
}

impl From<&LoveFifteen> for LoveThirty {
    fn from(_: &LoveFifteen) -> LoveThirty {
        LoveThirty
    }
}

impl From<&ThirtyLove> for ThirtyFifteen {
    fn from(_: &ThirtyLove) -> ThirtyFifteen {
        ThirtyFifteen
    }
}

impl From<&FifteenFifteen> for ThirtyFifteen {
    fn from(_: &FifteenFifteen) -> ThirtyFifteen {
        ThirtyFifteen
    }
}

impl From<&FifteenFifteen> for FifteenThirty {
    fn from(_: &FifteenFifteen) -> FifteenThirty {
        FifteenThirty
    }
}

impl From<&LoveThirty> for FifteenThirty {
    fn from(_: &LoveThirty) -> FifteenThirty {
        FifteenThirty
    }
}

impl From<&ThirtyFifteen> for ThirtyThirty {
    fn from(_: &ThirtyFifteen) -> ThirtyThirty {
        ThirtyThirty
    }
}

impl From<&FifteenThirty> for ThirtyThirty {
    fn from(_: &FifteenThirty) -> ThirtyThirty {
        ThirtyThirty
    }
}

impl From<&ThirtyLove> for FortyLove {
    fn from(_: &ThirtyLove) -> FortyLove {
        FortyLove
    }
}

impl From<&LoveThirty> for LoveForty {
    fn from(_: &LoveThirty) -> LoveForty {
        LoveForty
    }
}

impl From<&FortyLove> for FortyFifteen {
    fn from(_: &FortyLove) -> FortyFifteen {
        FortyFifteen
    }
}

impl From<&ThirtyFifteen> for FortyFifteen {
    fn from(_: &ThirtyFifteen) -> FortyFifteen {
        FortyFifteen
    }
}

impl From<&FifteenThirty> for FifteenForty {
    fn from(_: &FifteenThirty) -> FifteenForty {
        FifteenForty
    }
}

impl From<&LoveForty> for FifteenForty {
    fn from(_: &LoveForty) -> FifteenForty {
        FifteenForty
    }
}

impl From<&FortyFifteen> for FortyThirty {
    fn from(_: &FortyFifteen) -> FortyThirty {
        FortyThirty
    }
}

impl From<&ThirtyThirty> for FortyThirty {
    fn from(_: &ThirtyThirty) -> FortyThirty {
        FortyThirty
    }
}

impl From<&FifteenForty> for ThirtyForty {
    fn from(_: &FifteenForty) -> ThirtyForty {
        ThirtyForty
    }
}

impl From<&ThirtyThirty> for ThirtyForty {
    fn from(_: &ThirtyThirty) -> ThirtyForty {
        ThirtyForty
    }
}

impl From<&FortyThirty> for Duece {
    fn from(_: &FortyThirty) -> Duece {
        Duece
    }
}

impl From<&ThirtyForty> for Duece {
    fn from(_: &ThirtyForty) -> Duece {
        Duece
    }
}

impl From<&AdvantagePlayer1> for Duece {
    fn from(_: &AdvantagePlayer1) -> Duece {
        Duece
    }
}

impl From<&AdvantagePlayer2> for Duece {
    fn from(_: &AdvantagePlayer2) -> Duece {
        Duece
    }
}

impl From<&Duece> for AdvantagePlayer1 {
    fn from(_: &Duece) -> AdvantagePlayer1 {
        AdvantagePlayer1
    }
}

impl From<&Duece> for AdvantagePlayer2 {
    fn from(_: &Duece) -> AdvantagePlayer2 {
        AdvantagePlayer2
    }
}

impl From<&FortyLove> for Player1Won {
    fn from(_: &FortyLove) -> Player1Won {
        Player1Won
    }
}

impl From<&FortyFifteen> for Player1Won {
    fn from(_: &FortyFifteen) -> Player1Won {
        Player1Won
    }
}

impl From<&FortyThirty> for Player1Won {
    fn from(_: &FortyThirty) -> Player1Won {
        Player1Won
    }
}

impl From<&AdvantagePlayer1> for Player1Won {
    fn from(_: &AdvantagePlayer1) -> Player1Won {
        Player1Won
    }
}

impl From<&LoveForty> for Player2Won {
    fn from(_: &LoveForty) -> Player2Won {
        Player2Won
    }
}

impl From<&FifteenForty> for Player2Won {
    fn from(_: &FifteenForty) -> Player2Won {
        Player2Won
    }
}

impl From<&ThirtyForty> for Player2Won {
    fn from(_: &ThirtyForty) -> Player2Won {
        Player2Won
    }
}

impl From<&AdvantagePlayer2> for Player2Won {
    fn from(_: &AdvantagePlayer2) -> Player2Won {
        Player2Won
    }
}

impl From<&Error> for Error {
    fn from(_: &Error) -> Error {
        Error
    }
}

impl From<&Player1Won> for Error {
    fn from(_: &Player1Won) -> Error {
        Error
    }
}

impl From<&Player2Won> for Error {
    fn from(_: &Player2Won) -> Error {
        Error
    }
}

enum Score {
    LoveLove(LoveLove),
    LoveFifteen(LoveFifteen),
    LoveThirty(LoveThirty),
    LoveForty(LoveForty),
    FifteenLove(FifteenLove),
    FifteenFifteen(FifteenFifteen),
    FifteenThirty(FifteenThirty),
    FifteenForty(FifteenForty),
    ThirtyLove(ThirtyLove),
    ThirtyFifteen(ThirtyFifteen),
    ThirtyThirty(ThirtyThirty),
    ThirtyForty(ThirtyForty),
    FortyLove(FortyLove),
    FortyFifteen(FortyFifteen),
    FortyThirty(FortyThirty),
    Duece(Duece),
    AdvantagePlayer1(AdvantagePlayer1),
    AdvantagePlayer2(AdvantagePlayer2),
    Player1Won(Player1Won),
    Player2Won(Player2Won),
    Error(Error),
}

struct Game {
    score: Score,
}

impl Game {
    fn new() -> Game {
        Game {
            score: Score::LoveLove(LoveLove),
        }
    }

    fn print_score(&self) {
        match self.score {
            Score::LoveLove(_) => println!("Love, Love"),
            Score::LoveFifteen(_) => println!("Love, Fifteen"),
            Score::LoveThirty(_) => println!("Love, Thirty"),
            Score::LoveForty(_) => println!("Love, Forty"),
            Score::FifteenLove(_) => println!("Fifteen, Love"),
            Score::FifteenFifteen(_) => println!("Fifteen, Fifteen"),
            Score::FifteenThirty(_) => println!("Fifteen, Thirty"),
            Score::FifteenForty(_) => println!("Fifteen, Forty"),
            Score::ThirtyLove(_) => println!("Thirty, Love"),
            Score::ThirtyFifteen(_) => println!("Thirty, Fifteen"),
            Score::ThirtyThirty(_) => println!("Thirty, Thirty"),
            Score::ThirtyForty(_) => println!("Thirty, Forty"),
            Score::FortyLove(_) => println!("Forty, Love"),
            Score::FortyFifteen(_) => println!("Forty, Fifteen"),
            Score::FortyThirty(_) => println!("Forty, Thirty"),
            Score::Duece(_) => println!("Duece"),
            Score::AdvantagePlayer1(_) => println!("Advantage Player 1"),
            Score::AdvantagePlayer2(_) => println!("Advantage Player 2"),
            Score::Player1Won(_) => println!("Player 1 Wins!"),
            Score::Player2Won(_) => println!("Player 2 Wins!"),
            Score::Error(_) => println!("Invalid point"),
        }
    }

    fn winner(&self) -> Option<Player> {
        match self.score {
            Score::Player1Won(_) => Some(Player::Player1),
            Score::Player2Won(_) => Some(Player::Player2),
            _ => None,
        }
    }

    fn play(mut self, points: &[Player]) -> Result<Option<Player>, &str> {
        self.print_score();

        for point in points.iter() {
            self = self.point(&point);
            self.print_score();
        }

        let winner = self.winner();
        if winner.is_some() {
            return Ok(winner);
        }

        if let Score::Error(_) = self.score {
            return Err("Error: points detected after game already won.");
        }

        Ok(None)
    }

    fn point(self, player: &Player) -> Game {
        match self.score {
            Score::LoveLove(s) => s.point(player),
            Score::LoveFifteen(s) => s.point(player),
            Score::LoveThirty(s) => s.point(player),
            Score::LoveForty(s) => s.point(player),
            Score::FifteenLove(s) => s.point(player),
            Score::FifteenFifteen(s) => s.point(player),
            Score::FifteenThirty(s) => s.point(player),
            Score::FifteenForty(s) => s.point(player),
            Score::ThirtyLove(s) => s.point(player),
            Score::ThirtyFifteen(s) => s.point(player),
            Score::ThirtyThirty(s) => s.point(player),
            Score::ThirtyForty(s) => s.point(player),
            Score::FortyLove(s) => s.point(player),
            Score::FortyFifteen(s) => s.point(player),
            Score::FortyThirty(s) => s.point(player),
            Score::Duece(s) => s.point(player),
            Score::AdvantagePlayer1(s) => s.point(player),
            Score::AdvantagePlayer2(s) => s.point(player),
            Score::Player1Won(s) => s.point(player),
            Score::Player2Won(s) => s.point(player),
            Score::Error(s) => s.point(player),
        }
    }
}

fn main() {
    let points = [
        Player::Player1,
        Player::Player2,
        Player::Player1,
        Player::Player2,
        Player::Player1,
        Player::Player2,
        Player::Player1,
        Player::Player2,
        Player::Player1,
        Player::Player1,
    ];

    let game = Game::new();
    let winner = game.play(&points);
    match winner {
        Ok(None) => println!("No winner, incomplete game."),
        Err(msg) => println!("{msg}"),
        _ => {}
    }
}
