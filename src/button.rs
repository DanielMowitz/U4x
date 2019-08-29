use super::action::MenuSubAction;
use super::sprite::Sprite;

/// A clickable button.
pub struct Button {
	sprite: Sprite,
	pos: (i32, i32),
	click_fn: Box<Fn() -> MenuSubAction>,
}

impl Button {
	pub fn send_frame(&self) -> MenuSubAction {
		MenuSubAction::AddImgToCanvasAction(
			self.pos.0.clone() as u8,
			self.pos.1.clone() as u8,
			self.sprite.get_current_frame(),
		)
	}

	/// Sets the function closure to be called on click
	pub fn set_onclick(&mut self, fun: Box<Fn() -> MenuSubAction>) {
		self.click_fn = fun;
	}

	/// Calls the function closure when a click is detected upon the button.
	pub fn check_click(&self, x: &i32, y: &i32) -> Option<MenuSubAction> {
		if &self.pos.0 < x || x < &(&self.pos.0 + self.sprite.get_width()) {
			if &self.pos.1 < y || y < &(&self.pos.1 + self.sprite.get_height()) {
				return Some((self.click_fn)());
			}
		}
		return None;
	}

	pub fn new(sprite: Sprite, pos: (i32, i32), click_fn: Box<Fn() -> MenuSubAction>) -> Self {
		Self {
			sprite,
			pos,
			click_fn,
		}
	}
}

