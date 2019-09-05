use sdl2::mouse::MouseButton;

use action::{Action, MenuSubAction};
use store;

pub trait Scene<'a> {
	//elements: vec<frame::game_obj::GameObj>
	fn receive_menu_sub_action(&mut self, sub_action: &super::action::MenuSubAction) -> super::store::ReceiveMSAReturnOption;
}

pub struct MinimalScene {
	button: super::button::Button
}

impl MinimalScene {
	pub fn new(button: super::button::Button) -> Self {
		Self {
			button
		}
	}
}

impl<'a> Scene<'a> for MinimalScene {
	fn receive_menu_sub_action(&mut self, sub_action: &super::action::MenuSubAction) -> super::store::ReceiveMSAReturnOption {
		match sub_action {
			super::action::MenuSubAction::DrawAction => {
				return super::store::ReceiveMSAReturnOption::NewAction(
					vec!(
						self.button.send_frame(), //todo:hier testen
					),
					self,
				);
			}

			super::action::MenuSubAction::ClickAction(x, y, mouse_btn) => {
				match mouse_btn {
					MouseButton::Left => match self.button.check_click(x, y) {
						Some(msa) => return super::store::ReceiveMSAReturnOption::NewAction(vec![msa], self),
						None => return super::store::ReceiveMSAReturnOption::NoNewAction(self)
					},
					_ => return super::store::ReceiveMSAReturnOption::NoNewAction(self)
				}
			}

			_ => {
				return super::store::ReceiveMSAReturnOption::NoNewAction(self);
			}
		}
	}
}

/// Very similar to the dispatcher, the SceneManager supervises menus.
pub struct SceneManager<'a> {
	scenes: Option<Vec<Option<&'a mut Scene<'a>>>>,
	//For an explaination of this look below. todo:better comment lol
	current_scene: usize,
}

impl<'a> SceneManager<'a> {
	pub fn new() -> Self {
		Self {
			scenes: None,
			current_scene: 0,
		}
	}

	pub fn add_scenes(&mut self, references: Vec<&'a mut Scene<'a>>) {
		let mut local_scenes: Vec<Option<&'a mut Scene<'a>>>;

		match self.scenes.take() {
			Some(x) => {
				local_scenes = x;
			}
			_ => {
				local_scenes = vec![];
			}
		}

		for reference in references {
			local_scenes.push(Some(reference));
		}

		self.scenes = Some(local_scenes);
	}
}

impl<'a> super::store::Store<'a> for SceneManager<'a> {
	fn receive_action(&'a mut self, action: &super::action::Action, dt: &f64) -> super::store::ReceiveActionReturnOption {
		match action {
			super::action::Action::MenuAction(menu_sub_action) => {
				let mut out_actions: Vec<Action> = vec![];

				match menu_sub_action {
					MenuSubAction::ChangeMenuStateAction => {
						out_actions.push(
							Action::MenuAction(MenuSubAction::WaitForInputAction)
						);
					}
					MenuSubAction::WaitForInputAction => {
						out_actions.push(
							Action::MenuAction(MenuSubAction::DrawAction)
						);
					}
					MenuSubAction::DrawAction => { //todo: make it actually draw stuff
						out_actions.push(
							Action::MenuAction(MenuSubAction::WaitForInputAction)
						);
					}
					_ => {}
				}

				match self.scenes.take() {
					//Psyche! Look at the similar function in dispatcher.
					Some(mut local_scenes) => {
						for index in 0..local_scenes.len() {
							match local_scenes[index].take() {
								Some(scene) => {
									match scene.receive_menu_sub_action(menu_sub_action) {
										store::ReceiveMSAReturnOption::NewAction(out_msa_vec, out_reference) => {
											for msa in out_msa_vec {
												out_actions.push(Action::MenuAction(msa));
											}
											local_scenes[index] = Some(out_reference);
										}
										store::ReceiveMSAReturnOption::NoNewAction(out_reference) => {
											local_scenes[index] = Some(out_reference);
										}
									}
								}
								None => println!("There is a missing scene...")
							}
						}

						self.scenes = Some(local_scenes);
						super::store::ReceiveActionReturnOption::NewAction(out_actions, false, self)
					}
					None => {
						panic!("No scenes yet ( ͡° ͜ʖ ͡°)");
						super::store::ReceiveActionReturnOption::NoNewAction(self)
					}
				}
			}
			_ => { super::store::ReceiveActionReturnOption::NoNewAction(self) }
		}
	}
}
