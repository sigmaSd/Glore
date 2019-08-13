//! # glore at log
//!
//! **HowTo:**
//!
//! 1- Use `glore::GLORE` at the root of your project
//!
//! 2- Add a log target with `glore::init($target)` `$target` is anything that impl `Write`
//!
//! 3- log!
//!
//! *example of usage:*
//!
//! ```rust
//! use glore::{init, log, GLORE};
//! # fn main() {
//!
//!	let f = std::fs::OpenOptions::new()
//! 	.append(true)
//! 	.open("log.txt")
//! 	.unwrap();
//! let stdout = std::io::stdout();
//!
//! init(stdout);
//! log!("hello ====");
//! log!("world");
//!
//! init(f);
//! log!("hello ====");
//! std::thread::spawn(|| {
//! 	log!("world");
//! })
//! .join();
//! # }
//! ```
//!

use std::io::Write;
/// Use this static at the root of your project to enable logging
///
/// `use glore::GLORE;`
pub static mut GLORE: Option<&mut Write> = None;

/// Use this function to add a log target
/// ```rust
///	let stdout = std::io::stdout();
///	glore::init(stdout);
/// ```
pub fn init(w: impl Write + 'static) {
    unsafe {
        let l = Box::new(w);
        let leaked = Box::leak(l);
        GLORE = Some(leaked);
    }
}

#[macro_export]
/// Use this macro log any argrument
///
/// `log!("log one: {}", 1);`
macro_rules! log {
    ($($arg:tt)*) => (
    	std::sync::Once::new().call_once(||{
   		use crate::GLORE;

   	    unsafe {
   	    	if let Some(writer) = GLORE.as_mut() {
   	  			match writeln!(writer, "{} line {}: {}", file!(), line!(), format_args!($($arg)*)) {
   	  				Ok(_) => (),
   	  				Err(e) => {panic!("Error writing to logger\nreason: {}", e);}
   	  			}
   	    	} else {
   	    		panic!("Nothing to log to yet, use init() before logging");
   	    	}
   	    }
   	    });
    );
}

#[cfg(test)]
mod tests {
    use super::{init, log};
    #[test]
    fn it_works() {
        let f = std::fs::OpenOptions::new()
            .append(true)
            .open("log.txt")
            .unwrap();
        let stdout = std::io::stdout();

        init(stdout);
        log!("hello ====");
        log!("world");

        init(f);
        log!("hello ====");
        let _ = std::thread::spawn(|| {
            log!("world");
        })
        .join();
    }
}
