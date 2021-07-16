use crate::KeybdKey;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use uinput::event::keyboard::Key;
use uinput::event::relative::Position;
use uinput::event::Code;

enum KeybdAction {
    Press,
    Release,
    Click,
}

impl crate::Key for KeybdKey {
    fn press(&self) {
        send_key_stroke(KeybdAction::Press, *self);
    }

    fn release(&self) {
        send_key_stroke(KeybdAction::Release, *self);
    }

    fn click(&self) {
        send_key_stroke(KeybdAction::Click, *self);
    }

    fn is_pressed(&self) -> bool {
        todo!()
    }

    fn is_toggled(&self) -> bool {
        todo!()
    }
}

fn send_key_stroke(action: KeybdAction, key: KeybdKey) {
    let mut device = device();
    if let Some(key) = key_to_event(key) {
        match action {
            KeybdAction::Press => device.press(&key).unwrap(),
            KeybdAction::Release => device.release(&key).unwrap(),
            KeybdAction::Click => device.click(&key).unwrap(),
        }
    }
    device.synchronize().unwrap();
}

fn device() -> MutexGuard<'static, uinput::Device> {
    lazy_static::lazy_static! {
        static ref DEVICE: Arc<Mutex<uinput::Device>> = {
            let device = Arc::new(Mutex::new(
                uinput::default()
                .unwrap()
                .name("mki")
                .unwrap()
                .event(uinput::event::Keyboard::All)
                .unwrap()
                .event(Position::X)
                .unwrap()
                .event(Position::Y)
                .unwrap()
                .create()
                .unwrap()));
            // Without this there seems to be some inputs gone to hell
            device.lock().unwrap().synchronize().unwrap();
            std::thread::sleep(Duration::from_millis(100));
            device
        };
    }
    DEVICE.lock().unwrap()
}

fn is_pressed() {}

pub fn key_to_event(key: KeybdKey) -> Option<Key> {
    use KeybdKey::*;
    match key {
        Backspace => Some(Key::BackSpace),
        Tab => Some(Key::Tab),
        Enter => Some(Key::Enter),
        Escape => Some(Key::Esc),
        Space => Some(Key::Space),
        Home => Some(Key::Home),
        Left => Some(Key::Left),
        Up => Some(Key::Up),
        Right => Some(Key::Right),
        Down => Some(Key::Down),
        Insert => Some(Key::Insert),
        Delete => Some(Key::Delete),
        Numrow0 => Some(Key::_0),
        Numrow1 => Some(Key::_1),
        Numrow2 => Some(Key::_2),
        Numrow3 => Some(Key::_3),
        Numrow4 => Some(Key::_4),
        Numrow5 => Some(Key::_5),
        Numrow6 => Some(Key::_6),
        Numrow7 => Some(Key::_7),
        Numrow8 => Some(Key::_8),
        Numrow9 => Some(Key::_9),
        A => Some(Key::A),
        B => Some(Key::B),
        C => Some(Key::C),
        D => Some(Key::D),
        E => Some(Key::E),
        F => Some(Key::F),
        G => Some(Key::G),
        H => Some(Key::H),
        I => Some(Key::I),
        J => Some(Key::J),
        K => Some(Key::K),
        L => Some(Key::L),
        M => Some(Key::M),
        N => Some(Key::N),
        O => Some(Key::O),
        P => Some(Key::P),
        Q => Some(Key::Q),
        R => Some(Key::R),
        S => Some(Key::S),
        T => Some(Key::T),
        U => Some(Key::U),
        V => Some(Key::V),
        W => Some(Key::W),
        X => Some(Key::X),
        Y => Some(Key::Y),
        Z => Some(Key::Z),
        Numpad0 => Some(Key::_0),
        Numpad1 => Some(Key::_1),
        Numpad2 => Some(Key::_2),
        Numpad3 => Some(Key::_3),
        Numpad4 => Some(Key::_4),
        Numpad5 => Some(Key::_5),
        Numpad6 => Some(Key::_6),
        Numpad7 => Some(Key::_7),
        Numpad8 => Some(Key::_8),
        Numpad9 => Some(Key::_9),
        F1 => Some(Key::F1),
        F2 => Some(Key::F2),
        F3 => Some(Key::F3),
        F4 => Some(Key::F4),
        F5 => Some(Key::F5),
        F6 => Some(Key::F6),
        F7 => Some(Key::F7),
        F8 => Some(Key::F8),
        F9 => Some(Key::F9),
        F10 => Some(Key::F10),
        NumLock => Some(Key::NumLock),
        ScrollLock => Some(Key::ScrollLock),
        CapsLock => Some(Key::CapsLock),
        LShift => Some(Key::LeftShift),
        RShift => Some(Key::RightShift),
        LControl => Some(Key::LeftControl),
        F11 => Some(Key::F11),
        F12 => Some(Key::F12),
        F13 => Some(Key::F13),
        F14 => Some(Key::F14),
        F15 => Some(Key::F15),
        F16 => Some(Key::F16),
        F17 => Some(Key::F17),
        F18 => Some(Key::F18),
        F19 => Some(Key::F19),
        F20 => Some(Key::F20),
        F21 => Some(Key::F21),
        F22 => Some(Key::F22),
        F23 => Some(Key::F23),
        F24 => Some(Key::F24),
        RControl => Some(Key::RightControl),
        Other(_code) => None,
    }
}

pub(crate) fn code_to_key(code: u32) -> KeybdKey {
    use KeybdKey::*;
    match code as i32 {
        code if Key::BackSpace.code() == code => Backspace,
        code if Key::Tab.code() == code => Tab,
        code if Key::Enter.code() == code => Enter,
        code if Key::Esc.code() == code => Escape,
        code if Key::Space.code() == code => Space,
        code if Key::Home.code() == code => Home,
        code if Key::Left.code() == code => Left,
        code if Key::Up.code() == code => Up,
        code if Key::Right.code() == code => Right,
        code if Key::Down.code() == code => Down,
        code if Key::Insert.code() == code => Insert,
        code if Key::Delete.code() == code => Delete,
        code if Key::_0.code() == code => Numrow0,
        code if Key::_1.code() == code => Numrow1,
        code if Key::_2.code() == code => Numrow2,
        code if Key::_3.code() == code => Numrow3,
        code if Key::_4.code() == code => Numrow4,
        code if Key::_5.code() == code => Numrow5,
        code if Key::_6.code() == code => Numrow6,
        code if Key::_7.code() == code => Numrow7,
        code if Key::_8.code() == code => Numrow8,
        code if Key::_9.code() == code => Numrow9,
        code if Key::A.code() == code => A,
        code if Key::B.code() == code => B,
        code if Key::C.code() == code => C,
        code if Key::D.code() == code => D,
        code if Key::E.code() == code => E,
        code if Key::F.code() == code => F,
        code if Key::G.code() == code => G,
        code if Key::H.code() == code => H,
        code if Key::I.code() == code => I,
        code if Key::J.code() == code => J,
        code if Key::K.code() == code => K,
        code if Key::L.code() == code => L,
        code if Key::M.code() == code => M,
        code if Key::N.code() == code => N,
        code if Key::O.code() == code => O,
        code if Key::P.code() == code => P,
        code if Key::Q.code() == code => Q,
        code if Key::R.code() == code => R,
        code if Key::S.code() == code => S,
        code if Key::T.code() == code => T,
        code if Key::U.code() == code => U,
        code if Key::V.code() == code => V,
        code if Key::W.code() == code => W,
        code if Key::X.code() == code => X,
        code if Key::Y.code() == code => Y,
        code if Key::Z.code() == code => Z,
        code if Key::_0.code() == code => Numpad0,
        code if Key::_1.code() == code => Numpad1,
        code if Key::_2.code() == code => Numpad2,
        code if Key::_3.code() == code => Numpad3,
        code if Key::_4.code() == code => Numpad4,
        code if Key::_5.code() == code => Numpad5,
        code if Key::_6.code() == code => Numpad6,
        code if Key::_7.code() == code => Numpad7,
        code if Key::_8.code() == code => Numpad8,
        code if Key::_9.code() == code => Numpad9,
        code if Key::F1.code() == code => F1,
        code if Key::F2.code() == code => F2,
        code if Key::F3.code() == code => F3,
        code if Key::F4.code() == code => F4,
        code if Key::F5.code() == code => F5,
        code if Key::F6.code() == code => F6,
        code if Key::F7.code() == code => F7,
        code if Key::F8.code() == code => F8,
        code if Key::F9.code() == code => F9,
        code if Key::F10.code() == code => F10,
        code if Key::NumLock.code() == code => NumLock,
        code if Key::ScrollLock.code() == code => ScrollLock,
        code if Key::CapsLock.code() == code => CapsLock,
        code if Key::LeftShift.code() == code => LShift,
        code if Key::RightShift.code() == code => RShift,
        code if Key::LeftControl.code() == code => LControl,
        code if Key::F11.code() == code => F11,
        code if Key::F12.code() == code => F12,
        code if Key::F13.code() == code => F13,
        code if Key::F14.code() == code => F14,
        code if Key::F15.code() == code => F15,
        code if Key::F16.code() == code => F16,
        code if Key::F17.code() == code => F17,
        code if Key::F18.code() == code => F18,
        code if Key::F19.code() == code => F19,
        code if Key::F20.code() == code => F20,
        code if Key::F21.code() == code => F21,
        code if Key::F22.code() == code => F22,
        code if Key::F23.code() == code => F23,
        code if Key::F24.code() == code => F24,
        code if Key::RightControl.code() == code => RControl,
        code => Other(code),
    }
}
