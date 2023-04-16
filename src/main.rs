#![feature(daisogen_api)]
#![feature(once_cell)]
#![feature(let_chains)]

mod input;

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static ECHO: AtomicBool = AtomicBool::new(true);

fn main() {
    input::init();
    //std::daisogen::pd_set("tty_readline", readline as u64);

    loop {
        // This assumes there's a keyboard, which is usual but incorrect
        // TODO: std::daisogen::wait_for("kbd_get_high_key"), this way it works
        // as soon as it's connected
        let key = std::daisogen::pd_call0("kbd_get_high_key");
        if key == 0 {
            continue;
        }

        // Yes, this is a race condition; no one cares. Just do it ASAP
        // Best-effort is fine in this case, it's keyboard, it's slow
        let c = std::daisogen::pd_call1("kbd_key_to_char", key);

        let key = unsafe { &*(key as *const String) };
        match (*key).as_str() {
            "Delete" => {
                let back = input::backspace();

                if ECHO.load(Relaxed) && let Some(back) = back {
                    print!("\u{8} \u{8}"); // Back and remove
                    if back.len() > 0 {
                        print!("{} ", back); // Print the rest and a space
                        print!("{}", "\u{8}".repeat(back.len())); // And move back
                    }
                    io::stdout().flush().unwrap();
                }
            }
            "Return" => {
                input::enter();
                if ECHO.load(Relaxed) {
                    println!();
                }
            }
            _ => {
                if c == 0 {
                    // It's not a known character, too bad!
                    continue;
                }

                let c = char::from_u32(c as u32).unwrap();
                input::insert(c);

                if ECHO.load(Relaxed) {
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
}

/*extern "C" fn readline() -> usize {
    // TODO serialize string here
}*/
