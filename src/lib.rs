use evdev::EventType;
use evdev::InputEvent;
use evdev::KeyCode;
use libc;
use std::{fs::File, os::fd::AsRawFd};

pub(crate) fn fill_events(
    event_buf: &mut Vec<libc::input_event>,
    fd: &File,
) -> std::io::Result<usize> {
    event_buf.reserve(32);
    let spare_capacity = event_buf.spare_capacity_mut();
    let spare_capacity_size = std::mem::size_of_val(spare_capacity);

    // use libc::read instead of nix::unistd::read b/c we need to pass an uninitialized buf
    let res = unsafe {
        libc::read(
            fd.as_raw_fd(),
            spare_capacity.as_mut_ptr() as _,
            spare_capacity_size,
        )
    };
    let bytes_read = nix::errno::Errno::result(res)?;
    let num_read = bytes_read as usize / std::mem::size_of::<libc::input_event>();
    unsafe {
        let len = event_buf.len();
        event_buf.set_len(len + num_read);
    }
    Ok(num_read)
}

/// Fetches and returns events from the kernel ring buffer without doing synchronization on
/// SYN_DROPPED.
///
/// By default this will block until events are available. Typically, users will want to call
/// this in a tight loop within a thread.
pub fn fetch_events<'a>(
    event_buf: &'a mut Vec<libc::input_event>,
    fd: &'a File,
) -> std::io::Result<impl Iterator<Item = InputEvent> + 'a> {
    fill_events(event_buf, fd)?;
    Ok(event_buf.drain(..).map(InputEvent::from))
}

fn fd_write_all(fd: std::os::fd::BorrowedFd<'_>, mut data: &[u8]) -> nix::Result<()> {
    loop {
        match nix::unistd::write(fd.as_raw_fd(), data) {
            Ok(0) => return Ok(()),
            Ok(n) => data = &data[n..],
            Err(e) if e == nix::Error::EINTR => {}
            Err(e) => return Err(e),
        }
    }
}

pub(crate) unsafe fn cast_to_bytes<T: ?Sized>(mem: &T) -> &[u8] {
    std::slice::from_raw_parts(mem as *const T as *const u8, std::mem::size_of_val(mem))
}

pub fn write_events(fd: std::os::fd::BorrowedFd<'_>, events: &[InputEvent]) -> nix::Result<()> {
    let bytes = unsafe { cast_to_bytes(events) };
    fd_write_all(fd, bytes)
}

pub trait InterceptInput {
    fn get_ev(&self) -> &InputEvent;

    fn is_key(&self) -> bool {
        self.get_ev().event_type() == EventType::KEY
    }

    fn is_caps(&self) -> bool {
        self.key_code_to_enum() == KeyCode::KEY_CAPSLOCK
    }

    fn is_hjkl(&self) -> bool {
        match self.key_code_to_enum() {
            KeyCode::KEY_H | KeyCode::KEY_J | KeyCode::KEY_K | KeyCode::KEY_L => true,
            _ => false,
        }
    }

    fn key_code_to_enum(&self) -> KeyCode {
        KeyCode::new(self.get_ev().code())
    }

    fn debug(&self) {
        let state = {
            if self.get_ev().value() == 1 {
                "Down"
            } else if self.get_ev().value() == 2 {
                "Repeat"
            } else {
                "Up"
            }
        };
        println!("{:?} {:?}", self.key_code_to_enum(), state);
    }

    fn get_hjkl_equivalent(&self) -> InputEvent {
        let eq_code = match self.key_code_to_enum() {
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

    fn get_down_ev(&self) -> InputEvent {
        InputEvent::new(self.get_ev().event_type().0, self.get_ev().code(), 1)
    }
    fn get_up_ev(&self) -> InputEvent {
        InputEvent::new(self.get_ev().event_type().0, self.get_ev().code(), 0)
    }

    fn is_repeat(&self) -> bool {
        self.get_ev().value() == 2
    }

    fn is_down(&self) -> bool {
        self.get_ev().value() == 1
    }

    fn is_up(&self) -> bool {
        self.get_ev().value() == 0
    }

    fn write_to_buff_file(&self, fd: std::os::fd::BorrowedFd<'_>) {
        // if self.is_key() {
        //     self.debug();
        // }
        write_events(fd, &[self.get_ev().clone()])
            .expect("Failed writing event to he file descriptor");
    }
}

impl InterceptInput for InputEvent {
    fn get_ev(&self) -> &InputEvent {
        &self
    }
}

pub trait EventModify {
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

impl EventModify for KeyCode {
    fn get_key_code(&self) -> &KeyCode {
        self
    }
}
