use super::action::Action;
use super::action::MenuSubAction;
use super::dispatcher::Dispatcher;

pub enum ReceiveActionReturnOption<'a> {
	NoNewAction(&'a mut Store<'a>),
	NewAction(Vec<Action>, /*add to secondary*/ bool, &'a mut Store<'a>),
}

pub enum ReceiveMSAReturnOption<'a> {
	NoNewAction(&'a mut super::scene_manager::Scene<'a>),
	NewAction(Vec<MenuSubAction>, &'a mut super::scene_manager::Scene<'a>),
}

/// Turns any boring struct in a Store in the Flux pattern sense. These objects contain all the pro-
/// grams logic and communicate by ways of actions through the dispatcher.
pub trait Store<'a> {
	///The function that is called by the dispatcher to hand over a reference to an action.
	fn receive_action(&'a mut self, action: &Action, dt: &f64) -> ReceiveActionReturnOption<'a>;
}
