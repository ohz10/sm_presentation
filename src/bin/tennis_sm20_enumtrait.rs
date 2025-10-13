use enumtrait;

enum Player {
    Player1,
    Player2,
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

#[enumtrait::store(tennis_trait_store)]
trait Tennis {
    fn point(&self, player: &Player) -> Game;
    fn print(&self);
}

impl Tennis for LoveLove {
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

    fn print(&self) {
        println!("Love, Love");
    }
}

impl Tennis for LoveFifteen {
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

    fn print(&self) {
        println!("Love, Fifteen");
    }
}

impl Tennis for LoveThirty {
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

    fn print(&self) {
        println!("Love, Thirty");
    }
}

impl Tennis for LoveForty {
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

    fn print(&self) {
        println!("Love, Forty");
    }
}

impl Tennis for FifteenLove {
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

    fn print(&self) {
        println!("Fifteen, Love");
    }
}

impl Tennis for FifteenFifteen {
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

    fn print(&self) {
        println!("Fifteen, Fifteen");
    }
}

impl Tennis for FifteenThirty {
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

    fn print(&self) {
        println!("Fifteen, Thirty");
    }
}

impl Tennis for FifteenForty {
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

    fn print(&self) {
        println!("Fifteen, Forty");
    }
}

impl Tennis for ThirtyLove {
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

    fn print(&self) {
        println!("Thirty, Love");
    }
}

impl Tennis for ThirtyFifteen {
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

    fn print(&self) {
        println!("Thirty, Fifteen");
    }
}

impl Tennis for ThirtyThirty {
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

    fn print(&self) {
        println!("Thirty, Thirty");
    }
}

impl Tennis for ThirtyForty {
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

    fn print(&self) {
        println!("Thirty, Forty");
    }
}

impl Tennis for FortyLove {
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

    fn print(&self) {
        println!("Forty, Love");
    }
}

impl Tennis for FortyFifteen {
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

    fn print(&self) {
        println!("Forty, Fifteen");
    }
}

impl Tennis for FortyThirty {
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

    fn print(&self) {
        println!("Forty, Thirty");
    }
}

impl Tennis for Duece {
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

    fn print(&self) {
        println!("Duece");
    }
}

impl Tennis for AdvantagePlayer1 {
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

    fn print(&self) {
        println!("Advantage Player 1");
    }
}

impl Tennis for AdvantagePlayer2 {
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

    fn print(&self) {
        println!("Advantage Player 2");
    }
}

impl Tennis for Player1Won {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }

    fn print(&self) {
        println!("Player 1 Won!");
    }
}

impl Tennis for Player2Won {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }

    fn print(&self) {
        println!("Player 2 Won!");
    }
}

impl Tennis for Error {
    fn point(&self, _: &Player) -> Game {
        Game {
            score: Score::Error(self.into()),
        }
    }

    fn print(&self) {
        println!("Error detected");
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

#[enumtrait::quick_enum]
#[enumtrait::store(score_store)]
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

#[enumtrait::impl_trait(tennis_trait_store for score_store)]
impl Tennis for Score {
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

    fn winner(&self) -> Option<Player> {
        match self.score {
            Score::Player1Won(_) => Some(Player::Player1),
            Score::Player2Won(_) => Some(Player::Player2),
            _ => None,
        }
    }

    fn play(mut self, points: &[Player]) -> Result<Option<Player>, &str> {
        self.score.print();

        for point in points.iter() {
            self = self.point(&point);
            self.score.print();
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
        self.score.point(player)
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
