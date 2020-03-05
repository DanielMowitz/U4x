extern crate frame;
extern crate sdl2;

use frame::scene_manager::SceneManager;

mod main_menu;
mod space_ship;
mod light_path;

fn main() {

	let mut ship = space_ship::Spaceship::new();
	let mut renderer = frame::renderer::Renderer::new(512, 512, None);

	let mut menu = main_menu::MainMenu::new();
	let mut scene_manager = SceneManager::new();

	scene_manager.add_scenes(vec![&mut menu]);

	let mut disp = frame::dispatcher::Dispatcher::new(1.0 / 60.0);

	frame::game_loop(
		&mut disp,
		vec![
			&mut ship,
			&mut renderer,
			&mut scene_manager,
		],
	);
}
