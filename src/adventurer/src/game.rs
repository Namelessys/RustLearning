mod player;
use player::Player;

struct Game {
	player: Player,
}

impl Game {
	fn new() -> Self {
		Self {
			player: Player::new("TEST"),
		}
	}
}

pub fn main() {
	let mut game = init();
	run(&mut game);
}

fn init() -> Game {
	println!("Start initialization");	
	let game = Game::new();
	game
}

fn run(game: &mut Game) {
	game.player.greet();
	game.player.name_set("Test2");
	game.player.greet();
}