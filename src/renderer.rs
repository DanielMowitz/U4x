extern crate sdl2;

use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

use action::MenuSubAction;

use super::action::Action;
use super::img::Img;
use super::store::ReceiveActionReturnOption;
use super::store::Store;

/// This is where the framework talks to SDL2 and where the color space is stored.
pub struct Renderer {
	pub sdl_context: Sdl,
	event_pump: EventPump,
	video_subsystem: VideoSubsystem,
	canvas: Canvas<Window>,
	color_space: [Color; 15],
	pixel_size: u32,
}

impl Renderer {
	pub fn new(width: u32, height: u32, space: Option<[Color; 15]>) -> Self {
		let sdl_context = sdl2::init().unwrap();
		let event_pump = match sdl_context.event_pump() {
			Ok(pump) => pump,
			_ => panic!("no Event Pump!")
		};
		let video_subsystem = sdl_context.video().unwrap();
		let window = video_subsystem.window("rust-sdl2 demo", width, height)
			.position_centered()
			.build()
			.unwrap();
		let canvas = window.into_canvas().build().unwrap();
		let color_space: [Color; 15];
		match space {
			Some(space) => {
				color_space = space;
			}
			None => {
				// Example color space
				color_space = [
					Color::RGB(0x74, 0xDC, 0x20),
					Color::RGB(0xCE, 0xB2, 0x7E),
					Color::RGB(0xD4, 0x3B, 0x3E),
					Color::RGB(0xA6, 0x37, 0x4F),
					Color::RGB(0xC6, 0x6C, 0x45),
					Color::RGB(0xC2, 0x9F, 0x6E),
					Color::RGB(0xB7, 0xA6, 0x75),
					Color::RGB(0xA5, 0xA1, 0x75),
					Color::RGB(0xF5, 0xC3, 0x5C),
					Color::RGB(0xD5, 0x8E, 0x55),
					Color::RGB(0x8B, 0x84, 0x4C),
					Color::RGB(0xAC, 0xA4, 0x7C),
					Color::RGB(0xA3, 0x7F, 0x59),
					Color::RGB(0xB0, 0x6C, 0x4C),
					Color::RGB(0x40, 0x43, 0x37),
				];
			}
		}
		return Self {
			sdl_context,
			event_pump,
			video_subsystem,
			canvas,
			color_space,
			pixel_size: 8,
		}
	}

	#[test]
	pub fn test(&mut self, img: Img) -> bool {
		print!("This is the Test function!\n");
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
		self.canvas.present();
		let mut i = 0;
		'running: loop {
			i += 1;
			println!("Draw cycle: {}", i);
			self.add_to_canvas(0, 0, img.clone());
			self.draw_with_clear();
			sleep(Duration::new(1, 0));
			if i > 10 {
				break 'running;
			}
		}

		return true;
	}

	///Adds an image to the canvas in the desired location
	pub fn add_to_canvas(&mut self, x: u32, y: u32, i: Img) {
		let mask: u8 = 0b00001111;
		let alpha_value: u8 = 15;

		for (y_img, row) in i.enumerate() {
			for (x_img, pixel_pair) in row.iter().enumerate() {
				for i in 0..2 { //get each pixel of pair

					let pixel = (pixel_pair >> (4 * i)) & mask;

					if pixel != alpha_value {
						let pixel_value = pixel as usize;
						self.canvas.set_draw_color(self.color_space[pixel_value])
					} else {
						self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0))
					}

					/*The following line makes it so the pixel pairs are drawn side by side in the
					right order*/
					let x_i32 = (x_img * 2) as i32 + x as i32 + (1 - i);
					let y_i32 = y_img as i32 + y as i32;
					let drawing_rect = Rect::new(self.pixel_size as i32 * x_i32, self.pixel_size as i32 * y_i32, self.pixel_size, self.pixel_size);

					match self.canvas.fill_rect(drawing_rect) {
						Ok(_) => {},
						Err(e) => eprintln!("Could not fill shape:{}", e),
					}

					match self.canvas.draw_rect(drawing_rect) {
						Ok(_) => {},
						Err(e) => eprintln!("Could not draw shape:{}", e),
					}
				}
			}
		}
	}

	/// Draws a black background and then the canvas
	pub fn draw_with_clear(&mut self) {
		self.canvas.present();
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	/// Just draws the canvas
	pub fn draw_without_clear(&mut self) {
		self.canvas.present();
	}

	pub fn change_color_space(&mut self, space: [Color; 15]) {
		self.color_space = space;
	}

	pub fn get_canvas_size(&mut self) -> (u32, u32) {
		match self.canvas.output_size() {
			Ok(r) => (r.0, r.1),
			_ => (0, 0)
		}
	}

	/// This function is used for all user input (i.e. Mouse, Keyboard...)
	fn handle_inputs(&mut self) -> Vec<Action> {
		let mut out_vec: Vec<Action> = vec![];
		for event in self.event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => {
					out_vec.push(Action::MenuAction(MenuSubAction::QuitAction));
				}
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					out_vec.push(Action::MenuAction(MenuSubAction::ChangeMenuStateAction));
				}
				Event::KeyDown { scancode, ..} => {
					match scancode {
						Some(c) => out_vec.push(Action::KeyboardAction(c)),
						None => {}
					}
				}
				//maybe mouseaction will be implemented here too some day…
				_ => {}
			};
		};
		return out_vec;
	}

	/// Used mainly in menus to decrease memory usage while no new information is given
	fn wait_for_inputs(&mut self) -> Vec<Action> {
		let mut out_vec: Vec<Action> = vec![];
		match self.event_pump.wait_event() {
			Event::Quit { .. } => {
				out_vec.push(Action::QuitAction);
			}
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				out_vec.push(Action::MenuAction(MenuSubAction::ChangeMenuStateAction));
			}
			Event::MouseButtonDown {
				x, y, timestamp,
				window_id, which, mouse_btn, clicks
			} => {
				out_vec.push(Action::MenuAction(MenuSubAction::ClickAction(x, y, mouse_btn, self.pixel_size)));
			}
			_ => {}
		};
		return out_vec;
	}
}

impl<'a> Store<'a> for Renderer {
	fn receive_action(&'a mut self, action: &Action, dt: &f64) -> ReceiveActionReturnOption {
		match action {
			&Action::AddImgToCanvasAction(ref x, ref y, ref i) => {
				self.add_to_canvas(*x, *y, i.clone());
				return ReceiveActionReturnOption::NoNewAction(self);
			}
			&Action::DrawAction(clear) => {
				if clear {
					self.draw_with_clear();
				} else {
					self.draw_without_clear();
				}
				return ReceiveActionReturnOption::NoNewAction(self);
			}
			&Action::EndFrameAction => {
				let size = self.get_canvas_size();
				let mut out_vec = self.handle_inputs();
				out_vec.push(Action::UpdateAction);
				out_vec.push(Action::SendFrameAction(size.0, size.1, self.pixel_size));
				out_vec.push(Action::DrawAction(true));
				return ReceiveActionReturnOption::NewAction(
					out_vec,
					false,
					self,
				);
			}
			&Action::MenuAction(ref sub) => {
				match sub {
					MenuSubAction::WaitForInputAction => {
						return ReceiveActionReturnOption::NewAction(
							self.wait_for_inputs(),
							false,
							self,
						);
					}
					MenuSubAction::DrawAction => {
						self.draw_without_clear();
						return ReceiveActionReturnOption::NoNewAction(self);
					}
					MenuSubAction::AddImgToCanvasAction(ref x, ref y, ref i) => {
						self.add_to_canvas(*x, *y, i.clone());
						return ReceiveActionReturnOption::NoNewAction(self);
					}
					_ => ReceiveActionReturnOption::NoNewAction(self)
				}
			}
			_ => return ReceiveActionReturnOption::NoNewAction(self)
		}
	}
}

//todo: render layers?