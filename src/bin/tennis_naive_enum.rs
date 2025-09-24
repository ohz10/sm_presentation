#[derive(Debug, PartialEq)]
enum Score {
    Love,
    Fifteen,
    Thirty,
    Forty,
}

impl Score {
    fn next(&self) -> Self {
        match self {
            Score::Love => Score::Fifteen,
            Score::Fifteen => Score::Thirty,
            Score::Thirty => Score::Forty,
            Score::Forty => Score::Forty,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    Player1,
    Player2,
}

struct Game {
    score: (Score, Score),
    advantage: Option<Player>,
}

impl Game {
    fn new() -> Game {
        Game {
            score: (Score::Love, Score::Love),
            advantage: None,
        }
    }

    fn is_duece(&self) -> bool {
        Score::Forty == self.score.0 && Score::Forty == self.score.1
    }

    fn advantage(&mut self, player: &Player) -> Option<Player> {
        println!("Advantage {player:#?}");
        self.advantage = Some(*player);
        self.advantage
    }

    fn on_duece(&mut self, point: &Player) -> Option<Player> {
        match (&self.advantage, point) {
            (None, _) => self.advantage(point),
            (Some(Player::Player1), Player::Player1) => Some(Player::Player1),
            (Some(Player::Player1), Player::Player2) => {
                self.advantage = None;
                None
            }
            (Some(Player::Player2), Player::Player2) => Some(Player::Player2),
            (Some(Player::Player2), Player::Player1) => {
                self.advantage = None;
                None
            }
        }
    }

    fn point(&mut self, player: &Player) -> Option<Player> {
        if self.is_duece() {
            return self.on_duece(&player);
        }

        match player {
            Player::Player1 => {
                if Score::Forty == self.score.0 {
                    Some(Player::Player1)
                } else {
                    self.score.0 = self.score.0.next();
                    None
                }
            }
            Player::Player2 => {
                if Score::Forty == self.score.1 {
                    Some(Player::Player2)
                } else {
                    self.score.1 = self.score.1.next();
                    None
                }
            }
        }
    }

    fn play(&mut self, points: &[Player]) -> Option<Player> {
        for point in points.iter() {
            let winner = self.point(point);
            if winner.is_some() {
                return winner;
            }

            println!("{0:#?}, {1:#?}", self.score.0, self.score.1);
        }

        None
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
        Player::Player1,
    ];

    let mut game = Game::new();
    let winner = game.play(&points);
    match winner {
        Some(p) => println!("{p:#?} wins!"),
        None => println!("No winner"),
    }
}
