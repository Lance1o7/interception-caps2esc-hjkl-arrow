use interception_rs::prelude::*;

pub trait HelperMods: InterceptInput {
    fn is_caps(&self) -> bool {
        self.key_code() == KeyCode::KEY_CAPSLOCK
    }

    fn is_esc(&self) -> bool {
        self.key_code() == KeyCode::KEY_ESC
    }

    fn is_hjkl(&self) -> bool {
        match self.key_code() {
            KeyCode::KEY_H | KeyCode::KEY_J | KeyCode::KEY_K | KeyCode::KEY_L => true,
            _ => false,
        }
    }

    fn get_hjkl_equivalent(&self) -> InputEvent {
        let eq_code = match self.key_code() {
            KeyCode::KEY_H => KeyCode::KEY_LEFT,
            KeyCode::KEY_J => KeyCode::KEY_DOWN,
            KeyCode::KEY_K => KeyCode::KEY_UP,
            KeyCode::KEY_L => KeyCode::KEY_RIGHT,
            a => a,
        };
        if self.is_down() {
            eq_code.down()
        } else if self.is_repeat() {
            eq_code.repeat()
        } else {
            eq_code.up()
        }
    }
}

impl HelperMods for InputEvent {}

trait CapsHjklEventMods {
    fn get_key_code(&self) -> &KeyCode;

    fn up(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 0)
    }

    fn down(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 1)
    }

    fn repeat(&self) -> InputEvent {
        InputEvent::new(EventType::KEY.0, self.get_key_code().code(), 2)
    }
}
