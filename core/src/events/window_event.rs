
use crate::state::mouse::MouseButton;

use keyboard_types::{Code, Key};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorIcon {
    Arrow,
    NResize,
    EResize,
}


// Events generated by the application in response to OS events 
#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    // Emitted when a window is closed
    WindowClose,
    // Emitted when a window is opened
    WindowResize(f32, f32),
    // Emitted when a mouse button is pressed
    MouseDown(MouseButton),
    // Emitted when a mouse button is released
    MouseUp(MouseButton),
    // Emitted when the mouse cursor is moved
    MouseMove(f32, f32),
    // Emitted when the mouse scroll wheel is scrolled
    MouseScroll(f32, f32),
    // 
    MouseOver,
    // 
    MouseOut,
    // Emitted when a character is typed 
    CharInput(char),
    // Emitted when a keyboard key is pressed
    KeyDown(Code, Option<Key>),
    // Emitted when a keyboard key is released
    KeyUp(Code, Option<Key>),
    // Sets the mouse cursor
    SetCursor(CursorIcon),
    // Emitted when mouse events have been captured
    MouseCaptureEvent,
    // Emitted when mouse events have been released
    MouseCaptureOutEvent,
    // Emitted when an entity changes position or size (TODO: check if this includes margins + borders)
    GeometryChanged,
    Redraw,
    Restyle,
    Relayout,
}