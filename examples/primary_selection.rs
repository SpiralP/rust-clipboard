extern crate clipboard;

#[cfg(target_os = "linux")]
use clipboard::x11_clipboard::{Primary, X11ClipboardContext};

#[cfg(target_os = "linux")]
fn main() {
    use clipboard::ClipboardProvider;

    let mut ctx: X11ClipboardContext<Primary> = ClipboardProvider::new().unwrap();

    let the_string = b"Hello, world!";

    ctx.set_contents("UTF8_STRING", the_string).unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Primary selection is only available under linux!");
}
