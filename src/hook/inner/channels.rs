use std::sync::Mutex;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::hook::KeyCode;

pub struct HookChannels {
    keyboard_sender: Mutex<Sender<KeyCode>>,
    mouse_sender: Mutex<Sender<KeyCode>>,
    receiver: Mutex<Receiver<KeyCode>>,
}

impl HookChannels {
    pub fn new() -> HookChannels {
        let (s, r) = channel();
        HookChannels {
            keyboard_sender: Mutex::new(s.clone()),
            mouse_sender: Mutex::new(s.clone()),
            receiver: Mutex::new(r),
        }
    }

    pub fn send_key_code(&self, kc: KeyCode) -> Result<(), std::sync::mpsc::SendError<KeyCode>> {
        self.keyboard_sender.lock().unwrap().send(kc)
    }

    pub fn send_mouse_code(&self, mc: KeyCode) -> Result<(), std::sync::mpsc::SendError<KeyCode>>  {
        self.mouse_sender.lock().unwrap().send(mc)
    }

    pub fn try_recv(&self) -> Result<KeyCode, std::sync::mpsc::TryRecvError> {
        self.receiver.lock().unwrap().try_recv()
    }
}