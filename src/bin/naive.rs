type Score = u8;

#[derive(Clone,Debug)]
enum Player {
	Player1,
	Player2,
}

struct Game {
	score: (Score, Score),
}

impl Game {
	fn new() -> Game {
		Game{ score: (0, 0) }
	}

	fn is_duece(self: &Self) -> bool {
		self.score.0 >= 3 && self.score.1 >=3
	}

	fn on_duece(self: &mut Self) -> Option<Player> {
		if self.score.0 >= self.score.1 + 2 {
			return Some(Player::Player1);
		}

		if self.score.1 >= self.score.0 + 2 {
			return Some(Player::Player2);
		}

		print!("duece: ");
		return None;
	}

	fn if_winner(self: &mut Self) -> Option<Player> {
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

	fn point(self: &mut Self, player: &Player) -> Option<Player> {
		match player {
			Player::Player1 => self.score.0 += 1,
			Player::Player2 => self.score.1 += 1,
		}

		self.if_winner()
	}
}

fn main() {
	let mut game = Game::new();
	let points = [
		  Player::Player1
		, Player::Player2
		, Player::Player1
		, Player::Player2
		, Player::Player1
		, Player::Player2
		, Player::Player1
		, Player::Player1
	];

	for point in points.iter() {
		let winner = game.point(point);
		match winner {
			Some(Player::Player1) => println!("Player 1 wins {0} to {1}", game.score.0, game.score.1),
			Some(Player::Player2) => println!("Player 2 wins {1} to {0}", game.score.0, game.score.1),
			None => println!("{0} to {1}", game.score.0, game.score.1),
		}
	}
}

