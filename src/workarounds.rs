use bevy::{prelude::*, input::mouse::MouseButtonInput, app::Events};

pub fn clear_mouse_input_events(mut events: ResMut<Events<MouseButtonInput>>) {
    events.clear();
}