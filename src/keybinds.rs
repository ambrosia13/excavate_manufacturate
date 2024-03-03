use bevy::prelude::*;

#[derive(Resource)]
pub struct Keybinds {
    // Movement
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,

    // Interaction
    pub break_block: MouseButton,
    pub place_block: MouseButton,

    // Menu
    pub pause: KeyCode,
    pub exit: KeyCode,
}

impl Default for Keybinds {
    fn default() -> Self {
        use KeyCode::*;

        Self {
            forward: KeyW,
            back: KeyS,
            left: KeyA,
            right: KeyD,
            up: Space,
            down: ShiftLeft,

            break_block: MouseButton::Left,
            place_block: MouseButton::Right,

            pause: Escape,
            exit: Backspace,
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.init_resource::<Keybinds>();
}
