use crate::messages;

use std::io;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

static mut enigo: Option<Enigo> = None;

pub fn init() -> io::Result<()> {
    unsafe { enigo = Some(Enigo::new(&Settings::default()).unwrap()); }
    Ok(())
}

fn key_for_turn(turn: messages::Turn) -> Key {
    match turn {
        messages::Turn::Li => Key::L,
        messages::Turn::Ri => Key::R,
        messages::Turn::Di => Key::D,
        messages::Turn::Ui => Key::U,
        messages::Turn::Fi => Key::F,
        messages::Turn::Bi => Key::B,
        messages::Turn::L => Key::L,
        messages::Turn::R => Key::R,
        messages::Turn::D => Key::D,
        messages::Turn::U => Key::U,
        messages::Turn::F => Key::F,
        messages::Turn::B => Key::B,
    }
}

fn do_emit(e: &mut Enigo, ev: messages::Turn) -> io::Result<()> {
    const with_shift = ev.is_inverse();
    if with_shift {
        e.key(Key::Shift, Direction::Press);
    }
    e.key(key_for_turn(ev), Direction::Click);
    if with_shift {
        e.key(Key::Shift, Direction::Release);
    }
    Ok(())
}

fn uninitialized_err() -> io::Result<()> {
    println!("uinput not initialized, cannot emit key press");
    panic!();
}

pub fn emit(ev: messages::Turn) -> io::Result<()> {
    unsafe {
        do_emit(enigo.as_mut().unwrap(), ev)
    }
}

pub fn destroy() -> io::Result<()> {
    Ok(())
}
