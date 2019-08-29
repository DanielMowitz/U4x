use action::{Action, MenuSubAction};
use dispatcher::Dispatcher;
use img::Img;
use sprite::Sprite;
use store::ReceiveActionReturnOption;
use store::Store;

///A simple implementation of an animated game object
pub struct ExampleObj {
	sprite: Option<Sprite>,
	accumulated_dt: f64,
}

impl<'a> ExampleObj {
	pub fn new(sprite: Sprite) -> Self {
		Self {
			sprite: Some(sprite),
			accumulated_dt: 0.0,
		}
	}

	fn send_frame(&'a mut self, dt: f64) -> Action {
		let mut out_action = Action::EmptyAction;

		match self.sprite.take() {
			Some(mut sprite) => {
				sprite.animate(dt);
				out_action = Action::AddImgToCanvasAction(
					sprite.get_pos().0.clone(),
					sprite.get_pos().1.clone(),
					sprite.get_current_frame(),
				);
				self.sprite = Some(sprite);
			}
			None => {}
		}
		return out_action;
	}
}

impl<'a> Store<'a> for ExampleObj {
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
			&Action::UpdateAction => {
				self.accumulated_dt += dt.clone();
				match self.sprite.take() {
					Some(mut sprite) => {
						sprite.set_pos((
							(self.accumulated_dt * 300.0) as u8,
							(self.accumulated_dt * 300.0) as u8
						));
						self.sprite = Some(sprite);
					}
					None => {}
				}
				return ReceiveActionReturnOption::NoNewAction(self);
			}
			_ => return ReceiveActionReturnOption::NoNewAction(self)
		}
	}
}
