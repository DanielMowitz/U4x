use std::collections::VecDeque;
use std::time::Instant;

use sdl2::Sdl;

use action::Action::MenuAction;

use super::action::*;
use super::store::*;

pub enum MenuState {
	NotInMenu,
	InMenu(std::time::Duration),
}

///A Dispatcher in the Flux pattern sense. All traffic goes through here (in the form of Actions).
///
/// This implementation uses a primary and a secondary FiFo stack. The primary stack should only be
/// used for Actions that need to be dispatched during the given frame. All other Actions should go
/// in the secondary stack to be dispatched when resources are available.
/// All Actions sent by Menus should also be sent to the second stack.
pub struct Dispatcher<'a> {
	primary_action_queue: VecDeque<Action>,
	secondary_action_queue: VecDeque<Action>,
	pub store_refs: Option<Vec<Option<&'a mut Store<'a>>>>,
	//the double use of Option is necessary for memory safety. Long explanation in dispatch method.
	pub menu_state: MenuState,
	pub use_secondary: bool,
	max_stack_time: f64,
	current_stack_start_time: Instant,
	dt: f64,
}

impl<'a> Dispatcher<'a> {
	///Creates a new Dispatcher object.
	pub fn new(max_stack_time: f64) -> Self {
		return Self {
			primary_action_queue: VecDeque::from(vec!(Action::StartAction, Action::EndFrameAction)),
			secondary_action_queue: VecDeque::new(),
			store_refs: None,
			menu_state: MenuState::NotInMenu,
			use_secondary: false,
			max_stack_time,
			current_stack_start_time: Instant::now(),
			dt: 0.0,
		}
	}

	/// Works through the two stacks and dispatches the topmost
	/// action of the current stack. Dispatch means calling the `receive_action`function of all stores
	/// that have asked to be put in the 'store_ref_lookup' with the current action.
	pub fn dispatch(&mut self) -> bool {
		let in_action: Action;

		match self.get_in_action() {
			Some(a) => {
				in_action = a;
			}
			None => return true,
		}

		match in_action {
			Action::EndFrameAction => {
				self.use_secondary = false;
				self.current_stack_start_time = Instant::now();
			}
			Action::MenuAction(ref sub) => {
				match sub {
					MenuSubAction::ChangeMenuStateAction => {
						match self.menu_state {
							MenuState::InMenu(delta) => {
								self.current_stack_start_time = Instant::now() - delta;
								self.menu_state = MenuState::NotInMenu;
							}
							MenuState::NotInMenu => {
								self.menu_state = MenuState::InMenu(self.current_stack_start_time.elapsed())
							}
						}
					}
					MenuSubAction::QuitAction => {
						return false;
					}
					_ => (),
				};
			}
			Action::QuitAction => {
				return false;
			}
			_ => (),
		}

		match self.store_refs.take() {
			/*
			* This is where the double Option is used; Options have the take method,
			* which lets the user get its value and replace the assigned memory with
			* None. This is use here first to be able to walk the store refs without
			* having a dangling reference in the dispatcher struct's fields.
			*/
			Some(mut local_store_refs) => {
				for index in 0..local_store_refs.len() {
					match local_store_refs[index].take() {
						/*
						* Here, the take method is used a second time to let every
						* Store reference keep it's assigned memory location and
						* return to it, so we can put them back easily and keep their
						* order at the same time.
						*/
						Some(in_reference) => {
							self.dt = self.current_stack_start_time.elapsed().as_float_secs();
							match in_reference.receive_action(&in_action, &self.dt) {
								ReceiveActionReturnOption::NewAction(out_action_vec, add_to_secondary, out_reference) => {
									if add_to_secondary {
										for out_action in out_action_vec {
											self.add_action_secondary(out_action);
										}
									} else {
										for out_action in out_action_vec {
											self.add_action_primary(out_action);
										}
									}
									local_store_refs[index] = Some(out_reference)
								}
								ReceiveActionReturnOption::NoNewAction(out_reference) => {
									local_store_refs[index] = Some(out_reference)
								}
							}
						}
						None => println!("There's an empty store ref in your dispatcher, bröther ( ͡° ͜ʖ ͡°)")
					}
				}
				self.store_refs = Some(local_store_refs)
			}
			None => println!("No store refs yet ( ͡° ͜ʖ ͡°)")
		}

		return true;
	}

	/// Used in the sipatch function.
	/// returns the appropriate action from the appropriate Stack.
	fn get_in_action(&mut self) -> Option<Action> {
		if self.use_secondary {
			if self.current_stack_start_time.elapsed().as_float_secs() >= self.max_stack_time {
				return Some(Action::EndFrameAction);
			} else {
				match self.secondary_action_queue.pop_front() {
					Some(x) => {
						return Some(x);
					}
					None => {
						return Some(Action::EndFrameAction);
					}
				}
			}
		} else {
			match self.menu_state {
				MenuState::NotInMenu => {
					match self.primary_action_queue.pop_front() {
						Some(x) => {
							match x {
								Action::MenuAction(sub) => {
									match sub {
										MenuSubAction::ChangeMenuStateAction => {
											return Some(MenuAction(MenuSubAction::ChangeMenuStateAction));
										}
										_ => return None
									}
								}
								_ => return Some(x),
							}
						}
						None => {
							if self.current_stack_start_time.elapsed().as_float_secs() >= self.max_stack_time {
								return Some(Action::EndFrameAction);
							} else {
								self.use_secondary = true;
								return None;
							}
						}
					}
				}
				MenuState::InMenu(time) => {
					let mut remove_index: Option<usize> = None;
					for index in 0..(self.primary_action_queue.len() - 1) {
						match self.primary_action_queue[index] {
							Action::MenuAction(ref sub) => {
								remove_index = Some(index);
								break;
							}
							_ => (),
						};
					}
					match remove_index {
						Some(i) => match self.primary_action_queue.remove(i) {
							Some(action) => return Some(action),
							None => return None,
						}
						None => {
							return Some(Action::MenuAction(MenuSubAction::WaitForInputAction));
						}
					}
				}
			}
		}
		return None;
	}

	///Adds an action to the primary stack.
	pub fn add_action_primary(&mut self, action: Action) {
		self.primary_action_queue.push_back(action)
	}

	///Adds an action to the secondary stack.
	pub fn add_action_secondary(&mut self, action: Action) {
		self.secondary_action_queue.push_back(action)
	}

	///Used to enter all Store references before starting the dispatcher.
	pub fn enter_refs(&mut self, references: Vec<&'a mut Store<'a>>) {
		let mut local_store_refs: Vec<Option<&'a mut Store<'a>>>;

		match self.store_refs.take() {
			Some(x) => {
				local_store_refs = x;
			}
			_ => {
				local_store_refs = vec![];
			}
		}

		for reference in references {
			local_store_refs.push(Some(reference));
		}

		self.store_refs = Some(local_store_refs);
	}

	///Used to drop all Store references before ending the game loop
	pub fn drop_refs(&mut self) {
		match self.store_refs.take(){
			_ => {}
		}
	}
}
