// because we're going to use 'player as usize' to 
// index into our table, we need Copy
#[derive(Clone,Copy)]
enum Player {
    Player1,
    Player2,
}

// because we're going to use 'score as usize' to
// index into our table, we need Copy
#[derive(Clone,Copy,PartialEq)]
enum Score {
    LoveLove,
    LoveFifteen,
    LoveThirty,
    LoveForty,
    FifteenLove,
    FifteenFifteen,
    FifteenThirty,
    FifteenForty,
    ThirtyLove,
    ThirtyFifteen,
    ThirtyThirty,
    ThirtyForty,
    FortyLove,
    FortyFifteen,
    FortyThirty,
    Duece,
    AdvantagePlayer1,
    AdvantagePlayer2,
    Player1Won,
    Player2Won,
    Error,
}

const SCORES: [&str; 21] = [
    "Love, Love",
    "Love, Fifteen",
    "Love, Thirty",
    "Love, Forty",
    "Fifteen, Love",
    "Fifteen, Fifteen",
    "Fifteen, Thirty",
    "Fifteen, Forty",
    "Thirty, Love",
    "Thirty, Fifteen",
    "Thirty, Thirty",
    "Thirty, Forty",
    "Forty, Love",
    "Forty, Fifteen",
    "Forty, Thirty",
    "Duece",
    "Advantage Player 1",
    "Advantage Player 2",
    "Player 1 Won!",
    "Player 2 Won!",
    "Error detected",
];

const STATE_TRANSITIONS: [[Score; 2] ; 21] = [
    // ------------------------------------------------------------------------------------------------------
    // Point to Player1                             Point to Player2                           Current Score
    // ------------------------------------------------------------------------------------------------------
    [Score::FifteenLove,        Score::LoveFifteen],        // LoveLove
    [Score::FifteenFifteen,     Score::LoveThirty],         // LoveFifteen,
    [Score::FifteenThirty,      Score::LoveForty],          // LoveThirty,
    [Score::FifteenForty,       Score::Player2Won],         // LoveForty,
    [Score::ThirtyLove,         Score::FifteenFifteen],     // FifteenLove,
    [Score::ThirtyFifteen,      Score::FifteenThirty],      // FifteenFifteen,
    [Score::ThirtyThirty,       Score::FifteenForty],       // FifteenThirty,
    [Score::ThirtyForty,        Score::Player2Won],         // FifteenForty,
    [Score::FortyLove,          Score::ThirtyFifteen],      // ThirtyLove,
    [Score::FortyFifteen,       Score::ThirtyThirty],       // ThirtyFifteen,
    [Score::FortyThirty,        Score::ThirtyForty],        // ThirtyThirty,
    [Score::Duece,              Score::Player2Won],         // ThirtyForty,
    [Score::Player1Won,         Score::FortyFifteen],       // FortyLove,
    [Score::Player1Won,         Score::FortyThirty],        // FortyFifteen,
    [Score::Player1Won,         Score::Duece],              // FortyThirty,
    [Score::AdvantagePlayer1,   Score::AdvantagePlayer2],   // Duece,
    [Score::Player1Won,         Score::Duece],              // AdvantagePlayer1,
    [Score::Duece,              Score::Player2Won],         // AdvantagePlayer2,
    [Score::Error,              Score::Error],              // Player1Won,
    [Score::Error,              Score::Error],              // Player2Won,
    [Score::Error,              Score::Error],              // Error
];

impl Score {
    fn print(&self) {
        println!("{}", SCORES[*self as usize]);
    }

    fn point(&self, player: &Player) -> Score {
        STATE_TRANSITIONS[*self as usize][*player as usize]
    }
}

struct Game {
    score: Score,
}

impl Game {
    fn winner(&self) -> Option<Player> {
        match self.score {
            Score::Player1Won => Some(Player::Player1),
            Score::Player2Won => Some(Player::Player2),
            _ => None,
        }
    }

    fn play(mut self, points: &[Player]) -> Result<Option<Player>, &str> {
        self.score.print();

        for player in points.iter() {
            self.score = self.score.point(player);
            self.score.print();
        }

        let winner = self.winner();
        if winner.is_some() {
            return Ok(winner);
        }

        if Score::Error == self.score {
            return Err("Error: points detected after game already won.");
        }

        Ok(None)
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

    let game = Game{score: Score::LoveLove};
    let winner = game.play(&points);
    match winner {
        Ok(None) => println!("No winner, incomplete game."),
        Err(msg) => println!("{msg}"),
        _ => {}
    }
}
