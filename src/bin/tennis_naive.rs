#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

type Score = u8;
struct Game {
    score: (Score, Score),
}

impl Game {
    fn new() -> Game {
        Game { score: (0, 0) }
    }

    fn is_duece(&self) -> bool {
        self.score.0 >= 3 && self.score.1 >= 3
    }

    fn on_duece(&self) -> Option<Player> {
        if self.score.0 >= self.score.1 + 2 {
            return Some(Player::Player1);
        }

        if self.score.1 >= self.score.0 + 2 {
            return Some(Player::Player2);
        }

        return None;
    }

    fn point(&mut self, player: &Player) -> Option<Player> {
        match player {
            Player::Player1 => self.score.0 += 1,
            Player::Player2 => self.score.1 += 1,
        }

        if self.is_duece() {
            return self.on_duece();
        }

        if self.score.0 > 3 {
            return Some(Player::Player1);
        }

        if self.score.1 > 3 {
            return Some(Player::Player2);
        }

        return None;
    }

    fn play(&mut self, points: &[Player]) -> Option<Player> {
        for point in points.iter() {
            let winner = self.point(point);

            println!("{0:#?}, {1:#?}", self.score.0, self.score.1);
            if winner.is_some() {
                return winner;
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
        Player::Player1,
    ];

    let mut game = Game::new();
    let winner = game.play(&points);
    match winner {
        Some(p) => println!("{p:#?} wins!"),
        None => println!("No winner"),
    }
}
