use action::{Action, MenuSubAction};
use dispatcher::Dispatcher;
use img::Img;
use sprite::Sprite;
use store::ReceiveActionReturnOption;
use store::Store;

pub struct TestObj {
	///A game object for testing
	sprite: Option<Sprite>,
}

impl<'a> TestObj {
	pub fn new(sprite: Sprite) -> Self {
		Self {
			sprite: Some(sprite),
		}
	}

	fn send_frame(&'a mut self, dt: f64) -> Action {
		let mut out_action = Action::EmptyAction;

		match self.sprite.take() {
			Some(mut sprite) => {
				sprite.animate(dt);
				out_action = Action::AddImgToCanvasAction(
					sprite.get_pos().0,
					sprite.get_pos().1,
					sprite.get_current_frame(),
				);
				self.sprite = Some(sprite);
			}
			None => {}
		}
		return out_action;
	}
}

impl<'a> Store<'a> for TestObj {
	fn receive_action(&'a mut self, action: &Action, dt: &f64) -> ReceiveActionReturnOption<'a> {
		match action {
			&Action::SendFrameAction => {
				return ReceiveActionReturnOption::NewAction(
					vec!(
						self.send_frame(dt.clone())
					),
					false,
					self,
				);
			}
			_ => return ReceiveActionReturnOption::NoNewAction(self)
		}
	}
}
