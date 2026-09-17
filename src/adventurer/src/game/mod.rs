#[allow(unused)]
mod entity;
#[allow(unused)]
mod player;
mod traits;
#[allow(unused)]
mod room;

use player::Player;
use traits::{
	HasName,
	Greeter,
};
use room::Room;

struct Game {
	player: Player,
	room: Room,
}

impl Game {
	fn new() -> Self {
		Self {
			player: Player::new("Player"),
			room: Room::new("TestRoom"),
		}
	}
}

pub fn run() {
	let mut game = init();
	main_loop(&mut game);
}

fn init() -> Game {
	println!("Start initialization");	
	let game = Game::new();
	game
}

fn main_loop(game: &mut Game) {
	game.player.greet();
	game.player.name_set("Test2");
	game.player.greet();
	
	game.room.greet();
}