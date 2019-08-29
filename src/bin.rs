extern crate frame;
extern crate sdl2;

use std::fs::File;

use frame::img::Img;

/// An example use of the framework
fn main() {
	let mut example_img = frame::img::Img::new_from_u8(4, vec![]);

	match File::open("./resources/test_img.u4i") {
		Ok(f) => {
			example_img = Img::new_from_File(f);
		}
		Err(e) => {
			print!("{}", e);
		}
	}

	let example_sprite = frame::sprite::Sprite::new((0, 0), vec![example_img], vec![(0, 1)], 60.0);
	let mut example_game_obj = frame::example_obj::ExampleObj::new(example_sprite);
	let mut example_renderer = frame::renderer::Renderer::new(255, 255, None);
	let mut example_disp = frame::dispatcher::Dispatcher::new(1.0 / 60.0);

	frame::game_loop(
		&mut example_disp,
		vec![
			&mut example_game_obj,
			&mut example_renderer,
		],
	);
}
