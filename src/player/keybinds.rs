use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerKeybinds {
    pub forward: KeyCode,
    pub back: KeyCode,

    pub left: KeyCode,
    pub right: KeyCode,

    pub up: KeyCode,
    pub down: KeyCode,

    pub break_block: MouseButton,
    pub place_block: MouseButton,
}

impl Default for PlayerKeybinds {
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
        }
    }
}

pub fn setup_player_keybinds(mut commands: Commands) {
    commands.init_resource::<PlayerKeybinds>();
}
