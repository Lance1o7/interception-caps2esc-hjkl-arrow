// Create a virtual keyboard, just while this is running.
// Generally this requires root.

use caps2esc_hjkl::*;
use evdev::{Device, EventType, KeyEvent};
use evdev::{InputEvent, KeyCode};
use libc;
use std::collections::HashMap;
use std::os::fd::AsFd;
use std::thread::sleep;
use std::time::Duration;

// fn ev_clone(ev: &InputEvent, value: i32) -> InputEvent {
//     InputEvent::new(ev.event_type().0, ev.code(), value)
// }

fn main() -> std::io::Result<()> {
    // let mut dev = Device::open("/dev/input/event3").unwrap();

    // loop {
    //     for ev in dev.fetch_events().unwrap() {
    //         // dbg!(&ev);
    //         if ev.event_type() == EventType::KEY {
    //             println!("{ev:?}");
    //         }
    //     }
    // }
    // Ok(())
    let out_fd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/stdout")?;

    let in_fd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/stdin")?;

    // let in_intercept = std::fs::OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .open("/dev/input/event3")?;

    let in_kbd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/stdin")?;
    // .open("/dev/input/by-id/usb-413c_Dell_KB216_Wired_Keyboard-event-kbd")?;

    let mut event_buf: Vec<libc::input_event> = vec![];
    let mut last_event: InputEvent = InputEvent::new(EventType::KEY.0, 0, 0);
    loop {
        for ev in fetch_events_wrap(&mut event_buf, &in_kbd) {
            if !ev.is_key() {
                ev.write_to_buff_file(out_fd.as_fd());
                continue;
            }
            if ev.is_caps() && (ev.is_down() || ev.is_repeat()) {
                if last_event.is_repeat() {
                    sleep(Duration::from_micros(20000));
                    InputEvent::new(EventType::SYNCHRONIZATION.0, 0, 0)
                        .write_to_buff_file(out_fd.as_fd());
                    last_event.get_up_ev().write_to_buff_file(out_fd.as_fd());
                }
                'caps_down: loop {
                    for cde in fetch_events_wrap(&mut event_buf, &in_kbd) {
                        if !cde.is_key() {
                            cde.write_to_buff_file(out_fd.as_fd());
                            continue;
                        };
                        if cde.is_caps() && cde.is_up() {
                            KeyCode::KEY_ESC.down().write_to_buff_file(out_fd.as_fd());
                            sleep(Duration::from_micros(20000));
                            InputEvent::new(EventType::SYNCHRONIZATION.0, 0, 0)
                                .write_to_buff_file(out_fd.as_fd());
                            KeyCode::KEY_ESC.up().write_to_buff_file(out_fd.as_fd());
                            break 'caps_down;
                        } else {
                            cde.get_hjkl_equivalent().write_to_buff_file(out_fd.as_fd());
                            loop {
                                for chjkl in fetch_events_wrap(&mut event_buf, &in_kbd) {
                                    if !chjkl.is_key() {
                                        chjkl.write_to_buff_file(out_fd.as_fd());
                                        continue;
                                    };
                                    if chjkl.is_caps() {
                                        if chjkl.is_up() {
                                            if last_event.is_repeat() {
                                                sleep(Duration::from_micros(20000));
                                                InputEvent::new(EventType::SYNCHRONIZATION.0, 0, 0)
                                                    .write_to_buff_file(out_fd.as_fd());
                                                last_event
                                                    .get_up_ev()
                                                    .write_to_buff_file(out_fd.as_fd());
                                            }
                                            break 'caps_down;
                                        }
                                    }
                                    last_event = chjkl.get_hjkl_equivalent();
                                    last_event.write_to_buff_file(out_fd.as_fd());
                                }
                            }
                        }
                    }
                }
            } else {
                ev.write_to_buff_file(out_fd.as_fd());
                last_event = ev;
            }
        }
    }
}

fn fetch_events_wrap(
    event_buf: &mut Vec<libc::input_event>,
    kbd: &std::fs::File,
) -> Vec<InputEvent> {
    let fetched_events;
    {
        fetched_events = fetch_events(event_buf, kbd).unwrap().collect();
    }
    return fetched_events;
}
