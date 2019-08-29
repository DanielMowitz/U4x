use std::fmt::Error;
use std::fs::File;
use std::io::Read;

/// An Image defined by it's width and a vector of pixels, made up of 4-bit unsigned ints (stored
/// as u8 for memory efficiency). The image height is implied through the length of the vector.
pub struct Img {
	width: usize,
	current_px: isize,
	pixels: Vec<u8>,
}

impl Img {
	/// Creates new Img from u8 vector.
	pub fn new_from_u8(mut width: usize, pixels: Vec<u8>) -> Self {
		Self {
			width,
			current_px: 0,
			pixels,
		}
	}

	/// Reads file into Img.
	pub fn new_from_File(mut f: File) -> Self {
		let mut width = 0;
		let mut pixels = vec![];

		let mut buf: Vec<u8> = vec![];
		f.read_to_end(&mut buf);

		if buf.len() < 4 {
			return Self {
				width: 0,
				current_px: 0,
				pixels: vec![],
			};
		}

		let mut ctr = 0;

		for (num) in buf.iter() {
			if ctr >= 4 {
				pixels.push(*num);
			} else {
				width = width | ((buf[ctr] as usize) << (3 - ctr));
			}
			ctr += 1;
		}

		return Self {
			width,
			current_px: 0,
			pixels,
		};
	}

	/// Returns pixels of Img
	pub fn get_pixels(&self) -> &Vec<u8> {
		&self.pixels
	}

	/// Returns width of Img
	pub fn get_width(&self) -> &usize {
		&self.width
	}
}

impl Iterator for Img {
	type Item = (Vec<u8>);

	fn next(&mut self) -> Option<Vec<u8>> {
		let width = self.width / 2; //Converts between u4 width and u8 width, macht es auch quer.

		if self.current_px >= 0 {
			let current_px = self.current_px as usize;
			if self.pixels.len() - current_px > width {
				let new_px = current_px + width;
				let out_vec = self.pixels[current_px..new_px].to_vec();
				self.current_px = new_px as isize;
				Some(out_vec)
			} else if self.pixels.len() - current_px > 0 {
				let out_vec = self.pixels[current_px..].to_vec();
				self.current_px = -1;
				Some(out_vec)
			} else {
				self.current_px = 0;
				None
			}
		} else {
			self.current_px = 0;
			None
		}
	}
}

impl Clone for Img {
	fn clone(&self) -> Self {
		Self {
			width: self.width,
			current_px: self.current_px,
			pixels: self.pixels.clone(),
		}
	}
}
