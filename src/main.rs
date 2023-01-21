#![cfg(windows)]
use winapi::shared::minwindef::*;

fn LowLevelKeyboardProc(
    code: INT,
    word_param: WPARAM,
    long_aram: LPARAM,
    ) -> LRESULT {
        0 as LRESULT
    }
    
fn main() {
    println!("Hello, world!");
}
