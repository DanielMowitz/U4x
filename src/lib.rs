#![feature(duration_float)]

extern crate rand;
extern crate sdl2;

pub mod dispatcher;
pub mod action;
pub mod store;
pub mod renderer;
pub mod sprite;
pub mod test_object;
pub mod img;
pub mod button;
pub mod scene_manager;

#[cfg(test)]
mod tests;

/// "Front-end"-function of the dispatcher
pub fn game_loop<'a>(disp: & mut dispatcher::Dispatcher<'a>, store_refs: Vec<&'a mut store::Store<'a>>) {

		disp.enter_refs(store_refs);

		while disp.dispatch() {};

		disp.drop_refs();

}
