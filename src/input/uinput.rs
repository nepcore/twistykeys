use crate::messages;

use std::os::unix::fs::OpenOptionsExt;
use std::{fs::OpenOptions, io, thread, time::Duration};

use input_linux::{
    EventKind, EventTime, InputEvent, InputId, KeyEvent,
    SynchronizeEvent, SynchronizeKind, UInputHandle,
};
use nix::libc::O_NONBLOCK;

static mut uhandle: Option<UInputHandle<std::fs::File>> = None;

pub fn init() -> io::Result<()> {
    let uinput_file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_NONBLOCK)
        .open("/dev/uinput")?;
    let uh = UInputHandle::new(uinput_file);

    uh.set_evbit(EventKind::Key)?;
    uh.set_keybit(input_linux::Key::F)?;
    uh.set_keybit(input_linux::Key::B)?;
    uh.set_keybit(input_linux::Key::L)?;
    uh.set_keybit(input_linux::Key::R)?;
    uh.set_keybit(input_linux::Key::U)?;
    uh.set_keybit(input_linux::Key::D)?;
    uh.set_keybit(input_linux::Key::LeftShift)?;

    let input_id = InputId {
        bustype: input_linux::sys::BUS_USB,
        vendor: 0x1234,
        product: 0x5678,
        version: 0,
    };
    let device_name = b"QiYi SmartCube";
    uh.create(&input_id, device_name, 0, &[])?;
    thread::sleep(Duration::from_secs(1));
    unsafe { uhandle = Some(uh); }
    Ok(())
}

fn key_for_turn(turn: messages::Turn) -> input_linux::Key {
    match turn {
        messages::Turn::Li => input_linux::Key::L,
        messages::Turn::Ri => input_linux::Key::R,
        messages::Turn::Di => input_linux::Key::D,
        messages::Turn::Ui => input_linux::Key::U,
        messages::Turn::Fi => input_linux::Key::F,
        messages::Turn::Bi => input_linux::Key::B,
        messages::Turn::L => input_linux::Key::L,
        messages::Turn::R => input_linux::Key::R,
        messages::Turn::D => input_linux::Key::D,
        messages::Turn::U => input_linux::Key::U,
        messages::Turn::F => input_linux::Key::F,
        messages::Turn::B => input_linux::Key::B,
    }
}

fn do_emit(uh: &UInputHandle<std::fs::File>, ev: messages::Turn) -> io::Result<()> {
    const ZERO: EventTime = EventTime::new(0, 0);
    let with_shift = ev.is_inverse();
    let key = key_for_turn(ev);
    let mut events = Vec::new();
    if with_shift {
        events.push(InputEvent::from(KeyEvent::new(ZERO, input_linux::Key::LeftShift, input_linux::KeyState::PRESSED)).into_raw());
    }
    events.push(InputEvent::from(KeyEvent::new(ZERO, key, input_linux::KeyState::PRESSED)).into_raw());
    events.push(InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).into_raw());
    events.push(InputEvent::from(KeyEvent::new(ZERO, key, input_linux::KeyState::RELEASED)).into_raw());
    if with_shift {
        events.push(InputEvent::from(KeyEvent::new(ZERO, input_linux::Key::LeftShift, input_linux::KeyState::RELEASED)).into_raw());
    }
    events.push(InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).into_raw());
    uh.write(&events)?;
    Ok(())
}

fn uninitialized_err() -> io::Result<()> {
    println!("uinput not initialized, cannot emit key press");
    panic!();
}

pub fn emit(ev: messages::Turn) -> io::Result<()> {
    unsafe {
        match &uhandle {
            Some(uh) => do_emit(uh, ev),
            None => uninitialized_err()
        }
    }
}

pub fn destroy() -> io::Result<()> {
    unsafe {
        match &uhandle {
            Some(uh) => Ok(uh.dev_destroy()?),
            None => Ok(())
        }
    }
}
