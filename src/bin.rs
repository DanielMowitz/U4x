extern crate frame;
extern crate sdl2;

use frame::scene_manager::SceneManager;
use frame::renderer::Renderer;
use frame::dispatcher::Dispatcher;
use frame::example_obj::ExampleObj;

fn main() {

	let mut renderer = Renderer::new(512, 512, None);

	let mut scene_manager = SceneManager::new();

	let mut disp = Dispatcher::new(1.0 / 60.0);
	
	let mut obj = 

	frame::game_loop(
		&mut disp,
		vec![
			&mut ship,
			&mut renderer,
			&mut scene_manager,
		],
	);
}
