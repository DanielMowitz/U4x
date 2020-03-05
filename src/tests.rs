/// The tests file. Tests should be run with the --test-threads 1 option so they don't block the
/// SDL environment for each other.

use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;

use rand;
use sdl2::video::Window;

use action::MenuSubAction;
use button::Button;
use scene_manager::MinimalScene;
use scene_manager::SceneManager;
use test_object::TestObj;

use super::action::Action;
use super::dispatcher::Dispatcher;
use super::img::Img;
use super::renderer::Renderer;
use super::sprite::Sprite;
use super::store::ReceiveActionReturnOption;
use super::store::Store;

struct TestStore {
	in_num: u8,
	out_num: u8,
}

impl<'a> Store<'a> for TestStore {
	fn receive_action(&'a mut self, action: &Action, dt: &f64) -> ReceiveActionReturnOption<'a> {
		match action {
			&Action::TestAction(num) => {
				self.out_num = num;
				return ReceiveActionReturnOption::NoNewAction(self);
			}
			&Action::EndFrameAction => {
				return ReceiveActionReturnOption::NewAction(
					vec!(
						Action::TestAction(self.in_num)
					),
					false,
					self,
				);
			}
			_ => return ReceiveActionReturnOption::NoNewAction(self)
		}
	}
}

#[test]
fn test_dispatcher() {
	/*creates a TestStore instance that holds a random number. The number will be sent to the dispatcher
	through a TestAction. The dispatcher should return the value. It will be asserted if the action pay-
	load value equals the value owned by the TestStore instance*/

	let mut test_num = rand::random();

	while test_num == 0 {
		test_num = rand::random()
	}

	let mut test_store = TestStore { in_num: test_num, out_num: 0 };

	{
		let mut disp = Dispatcher::new(1.0 / 60.0);
		disp.enter_refs(vec!(&mut test_store));

		for i in 1..10 {
			disp.dispatch();
		}
	}

	assert_eq!(test_num, test_store.out_num)
}

#[test]
fn test_renderer() {
	/* creates a test game that owns a renderer and renders an image with all the colors of the color
	space*/
	let mut test_renderer: Renderer;
	test_renderer = Renderer::new(256, 256, None);
	let mut test_img = Img::new_from_u8(0, vec![]);

	match File::open("./resources/test_img.u4i") {
		Ok(f) => {
			test_img = Img::new_from_file(f);
		}
		Err(e) => {
			print!("{}", e);
		}
	}

	assert!(test_renderer.test(test_img));
}

#[test]
fn test_sprites_and_menus() {
	let mut test_renderer = Renderer::new(256, 256, None);
	let mut test_sprite = Sprite::new((0, 0), vec![], vec![], 0.0);
	let mut button_sprite = Sprite::new((0, 0), vec![], vec![], 0.0);

	match File::open("./resources/test_sprite.u4i") {
		Ok(f) => {
			test_sprite = Sprite::new_from_file(
				f,
				16,
				(0, 0),
				vec![(0, 12)],
				30.0,
			);
		}
		Err(e) => {
			println!("{}", e);
		}
	}

	let mut test_object = TestObj::new(test_sprite);

	match File::open("./resources/exit_button.u4i") {
		Ok(f) => {
			button_sprite = Sprite::new_from_file(
				f,
				133,
				(0, 0),
				vec![(0, 1)],
				1.0,
			);
		}
		Err(e) => {
			println!("{}", e);
		}
	}

	let mut test_button = Button::new(
		button_sprite,
		(0, 0),
		Box::new(|| {
			println!("Clicked button!");
			return MenuSubAction::QuitAction;
		}),
	);

	let mut test_scene = MinimalScene::new(
		test_button
	);

	let mut scene_manager = SceneManager::new();

	scene_manager.add_scenes(vec![&mut test_scene]);

	{
		let mut disp = Dispatcher::new(1.0);
		disp.enter_refs(vec!(&mut test_renderer, &mut test_object, &mut scene_manager));//, &mut test_object));

		while disp.dispatch() {};
	}
}
