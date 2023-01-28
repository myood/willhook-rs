use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::hook::event::*;

pub struct HookChannels {
    keyboard_sender: Mutex<Sender<InputEvent>>,
    mouse_sender: Mutex<Sender<InputEvent>>,
    receiver: Mutex<Receiver<InputEvent>>,
    counter: AtomicU64,
}

impl HookChannels {
    pub fn new() -> HookChannels {
        let (s, r) = channel();
        HookChannels {
            keyboard_sender: Mutex::new(s.clone()),
            mouse_sender: Mutex::new(s.clone()),
            receiver: Mutex::new(r),
            counter: AtomicU64::new(0),
        }
    }

    pub fn send_keyboard_event(&self, ke: KeyboardEvent) -> Result<(), std::sync::mpsc::SendError<InputEvent>> {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::Release);
        self.keyboard_sender.lock().unwrap().send(InputEvent::Keyboard(ke))
    }

    pub fn send_mouse_event(&self, me: MouseEvent) -> Result<(), std::sync::mpsc::SendError<InputEvent>>  {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::Release);
        self.mouse_sender.lock().unwrap().send(InputEvent::Mouse(me))
    }

    pub fn try_recv(&self) -> Result<InputEvent, std::sync::mpsc::TryRecvError> {
        let r = self.receiver.lock().unwrap().try_recv();
        if r.is_ok() {
            self.counter.fetch_sub(1, std::sync::atomic::Ordering::Release);
            println!("{}", self.counter.load(std::sync::atomic::Ordering::Acquire));
        }
        r
    }
}