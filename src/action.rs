use std::any::Any;

use sdl2::mouse::MouseButton;

use super::img::Img;

/// An Action in the Flux pattern sense. Has a type and a payload.
///
/// All communication between the stores happen through these babies. They are sent to the
/// dispatcher which sends them to every store that wants to hear of actions of their type.
pub enum Action {
	AddImgToCanvasAction(u8, u8, Img),
	DrawAction(bool),
	UpdateAction,
	SendFrameAction,
	EndFrameAction,
	StartAction,
	MenuAction(MenuSubAction),
	QuitAction,
	EmptyAction,
	TestAction(u8),
}

/// Similar to the action Enum. Only meant for use in menu states.
pub enum MenuSubAction {
	ChangeMenuStateAction,
	WaitForInputAction,
	DrawAction,
	AddImgToCanvasAction(u8, u8, Img),
	ClickAction(i32, i32, MouseButton),
	QuitAction,
}
