use std::fs::File;
use std::io::Read;

use super::img::Img;

pub struct Sprite {
	///A sprite with animations (using the function animate)
	pos: (u32, u32),
	frames: Vec<Img>,
	anims: Vec<(usize, usize)>,
	current_anim: usize,
	current_frame: usize,
	dt_since_last_frame: f64,
	framerate: f64,
}

/// Wrapper for the Img struct that handles animations and location.
impl Sprite {
	pub fn new(pos: (u32, u32), frames: Vec<Img>, anims: Vec<(usize, usize)>, framerate: f64) -> Self {
		Self {
			pos,
			frames,
			anims,
			current_anim: 0,
			current_frame: 0,
			dt_since_last_frame: 0.0,
			framerate,
		}
	}

	/// Updates the internal state of the animation
	pub fn animate(&mut self, dt: f64) {
		if self.dt_since_last_frame + dt >= (1.0 / self.framerate) {
			if self.current_frame + 1 < self.anims[self.current_anim].0 + self.anims[self.current_anim].1 {
				self.current_frame += 1;
			} else {
				self.current_frame = self.anims[self.current_anim].0
			}
			self.dt_since_last_frame = 0.0;
		} else {
			self.dt_since_last_frame += dt;
		}
	}

	pub fn get_pos(&self) -> (u32, u32) { return self.pos.clone() }

	pub fn set_pos(&mut self, pos: (u32, u32)) { self.pos = pos }

	pub fn get_width(&self) -> u32 {
		return self.get_current_frame().get_width().clone() as u32;
	}

	pub fn get_height(&self) -> u32 {
		return 2 * (self.get_current_frame().get_length() / *self.get_current_frame().get_width()) as u32;
	}

	pub fn get_current_frame(&self) -> Img {
		self.frames[self.current_frame].clone()
	}

	/// Wraps Img::new_from_file.
	pub fn new_from_file(mut f: File, pix_per_frame: usize, pos: (u32, u32), anims: Vec<(usize, usize)>, framerate: f64) -> Self {
		let mut width = 0;
		let mut pixels = vec![];
		let mut frames = vec![];
		let real_pix_per_frame = pix_per_frame / 2;

		let mut buf: Vec<u8> = vec![];
		match f.read_to_end(&mut buf) {
			Ok(_) => {},
			Err(e) => eprintln!("Could not read image buffer: {}", e),
		}

		if buf.len() < 4 {
			return Self {
				pos,
				frames: vec![],
				anims: vec![],
				current_anim: 0,
				current_frame: 0,
				dt_since_last_frame: 0.0,
				framerate: 0.0,
			};
		}

		let mut ctr = 0;

		for num in buf.iter() {
			if ctr >= 4 {
				if (ctr - 4) % real_pix_per_frame == 0 {
					pixels = vec![*num];
				} else if (ctr - 4) % real_pix_per_frame == real_pix_per_frame - 1 {
					pixels.push(*num);
					frames.push(
						Img::new_from_u8(width, pixels.clone())
					);
				} else {
					pixels.push(*num);
				}
			} else {
				width = width | ((buf[ctr] as usize) << (3 - ctr));
			}
			ctr += 1;
		}

		return Self {
			pos,
			frames,
			anims,
			current_anim: 0,
			current_frame: 0,
			dt_since_last_frame: 0.0,
			framerate,
		}
	}
}
