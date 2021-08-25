use std::borrow::Cow;

use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

use crate::input_handler::InputHandler;

/// If this event was for manipulating a keyboard key then this will return the `VirtualKeyCode`
/// and the new state.
#[must_use]
pub fn get_key(event: &Event<'_, ()>) -> Option<(VirtualKeyCode, ElementState)> {
    match *event {
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(ref virtual_keycode),
                            state,
                            ..
                        },
                    ..
                },
            ..
        } => Some((*virtual_keycode, state)),
        _ => None,
    }
}

/// Returns true if the event passed in is a key down event for the
/// provided `VirtualKeyCode`.
#[must_use]
pub fn is_key_down(event: &Event<'_, ()>, key_code: VirtualKeyCode) -> bool {
    if let Some((key, state)) = get_key(event) {
        return key == key_code && state == ElementState::Pressed;
    }

    false
}

/// Returns true if the event passed in is a key up event for the
/// provided `VirtualKeyCode`.
#[must_use]
pub fn is_key_up(event: &Event<'_, ()>, key_code: VirtualKeyCode) -> bool {
    if let Some((key, state)) = get_key(event) {
        return key == key_code && state == ElementState::Released;
    }

    false
}

/// Returns true if the event passed in is a request to close the game window.
#[must_use]
pub fn is_close_requested(event: &Event<'_, ()>) -> bool {
    matches!(
        *event,
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        }
    )
}

/// Gets the input axis value from the `InputHandler`.
/// If the name is None, it will return the default value of the axis (0.0).
#[must_use]
pub fn get_input_axis_simple(name: &Option<Cow<'static, str>>, input: &InputHandler) -> f32 {
    name.as_ref()
        .and_then(|n| input.axis_value(n))
        .unwrap_or(0.0)
}

/// Gets the action active status from the `InputHandler`.
/// If the action name is None, it will default to false.
#[must_use]
pub fn get_action_simple(name: &Option<Cow<'static, str>>, input: &InputHandler) -> bool {
    name.as_ref()
        .and_then(|n| input.action_is_down(n))
        .unwrap_or(false)
}

/// If this event was for manipulating a mouse button, this will return the `MouseButton`
/// and the new state.
#[must_use]
pub fn get_mouse_button(event: &Event<'_, ()>) -> Option<(MouseButton, ElementState)> {
    match *event {
        Event::WindowEvent {
            event: WindowEvent::MouseInput { button, state, .. },
            ..
        } => Some((button, state)),
        _ => None,
    }
}

/// Returns true if the event passed in is a mouse button down event for the
/// provided `MouseButton`.
#[must_use]
pub fn is_mouse_button_down(event: &Event<'_, ()>, button: MouseButton) -> bool {
    if let Some((pressed_button, state)) = get_mouse_button(event) {
        pressed_button == button && state == ElementState::Pressed
    } else {
        false
    }
}
