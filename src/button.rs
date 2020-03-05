use super::action::MenuSubAction;
use super::sprite::Sprite;

/// A clickable button.
pub struct Button {
	sprite: Sprite,
	click_fn: Box<Fn() -> MenuSubAction>,
}

impl Button {
	pub fn send_frame(&self) -> MenuSubAction {
		MenuSubAction::AddImgToCanvasAction(
			self.sprite.get_pos().0,
			self.sprite.get_pos().1,
			self.sprite.get_current_frame(),
		)
	}

	/// Sets the function closure to be called on click
	pub fn set_onclick(&mut self, fun: Box<Fn() -> MenuSubAction>) {
		self.click_fn = fun;
	}

	/// Calls the function closure when a click is detected upon the button.
	pub fn check_click(&self, x: u32, y: u32, px_size: u32) -> Option<MenuSubAction> {

		let sprite_x = &self.sprite.get_pos().0 * px_size;
		let sprite_y = &self.sprite.get_pos().1 * px_size;

		println!("cx:{}, cy:{}", sprite_x, sprite_y);
		println!("cx:{}, cy:{}", x, y);
		println!("cx:{}, cy:{}", sprite_x + self.sprite.get_width() * px_size, sprite_y + self.sprite.get_height() * px_size);

		if sprite_x < x && x < sprite_x + self.sprite.get_width() * px_size {
			if sprite_y < y && y < sprite_y + self.sprite.get_height() * px_size {
				return Some((self.click_fn)());
			}
		}
		return None;
	}

	pub fn new(sprite: Sprite, click_fn: Box<Fn() -> MenuSubAction>) -> Self {
		Self {
			sprite,
			click_fn,
		}
	}
}

