extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let the_string = b"Hello, world!";

    ctx.set_contents("UTF8_STRING", the_string).unwrap();
}
