#[derive(Clone, Copy, Debug, PartialEq)]
enum Score {
    Love,
    Fifteen,
    Thirty,
    Forty,
}

impl Score {
    fn next(self: &Self) -> Self {
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

#[derive(Clone, Copy)]
struct RegularScoringState {
    score: (Score, Score),
}

impl RegularScoringState {
    fn new() -> Self {
        Self {
            score: (Score::Love, Score::Love),
        }
    }

    fn point(self: &mut Self, player: Player) -> GameState {
        match player {
            Player::Player1 => self.score.0 = self.score.0.next(),
            Player::Player2 => self.score.1 = self.score.1.next(),
        };

        if Score::Forty == self.score.0 && self.score.0 == self.score.1 {
            return GameState::Duece(DueceState {});
        }

        GameState::RegularScoring(RegularScoringState { score: self.score })
    }
}

#[derive(Clone, Copy)]
struct AdvantageState {
    advantage: Player,
}

impl AdvantageState {
    fn new(player: Player) -> Self {
        Self { advantage: player }
    }

    fn point(self, player: Player) -> GameState {
        if self.advantage == player {
            return GameState::GameOver(GameOverState::new(player));
        }

        GameState::Duece(DueceState {})
    }
}

#[derive(Clone, Copy)]
struct DueceState {}

impl DueceState {
    fn point(self, player: Player) -> GameState {
        GameState::Advantage(AdvantageState::new(player))
    }
}

#[derive(Clone, Copy)]
struct GameOverState {
    winner: Player,
}

impl GameOverState {
    fn new(player: Player) -> Self {
        Self { winner: player }
    }

    fn point(self, _: Player) -> GameState {
        GameState::GameOver(self)
    }
}

enum GameState {
    RegularScoring(RegularScoringState),
    Duece(DueceState),
    Advantage(AdvantageState),
    GameOver(GameOverState),
}

struct Game {
    state: GameState,
}

impl Game {
    fn new() -> Game {
        Game {
            state: GameState::RegularScoring(RegularScoringState::new()),
        }
    }

    fn winner(self: &Self) -> Option<Player> {
        if let GameState::GameOver(GameOverState { winner }) = &self.state {
            return match winner {
                Player::Player1 => Some(Player::Player1),
                Player::Player2 => Some(Player::Player2),
            };
        }

        None
    }

    fn point(self: &mut Self, player: Player) -> GameState {
        match (&mut self.state, player) {
            (GameState::RegularScoring(s), _) => s.point(player),
            (GameState::Duece(s), _) => s.point(player),
            (GameState::Advantage(s), _) => s.point(player),
            (GameState::GameOver(s), _) => s.point(player),
        }
    }

    fn play(self: &mut Self, points: &[Player]) -> Option<Player> {
        for &point in points.iter() {
            self.state = self.point(point);

            let winner = self.winner();
            if winner.is_some() {
                return winner;
            }

            match self.state {
                GameState::RegularScoring(RegularScoringState { score }) => {
                    println!("{0:#?}, {1:#?}", score.0, score.1)
                }
                GameState::Advantage(AdvantageState { advantage }) => {
                    println!("Advantage {advantage:#?}")
                }
                GameState::Duece(_) => println!("Duece"),
                GameState::GameOver(GameOverState { winner }) => println!("{winner:#?} wins!"),
            }
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
        Player::Player2,
        Player::Player1,
        Player::Player1,
    ];

    let mut game = Game::new();
    let winner = game.play(&points);
    match winner {
        Some(Player::Player1) => println!("Player 1 wins!"),
        Some(Player::Player2) => println!("Player 2 wins!"),
        None => println!("No winner"),
    }
}
