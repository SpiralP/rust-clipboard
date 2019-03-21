/*
Copyright 2016 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use clipboard_win::Clipboard;

use common::ClipboardProvider;
use std::error::Error;

pub struct WindowsClipboardContext;

impl ClipboardProvider for WindowsClipboardContext {
    fn new() -> Result<Self, Box<Error>> {
        Ok(WindowsClipboardContext)
    }
    fn get_contents(&mut self) -> Result<Vec<u8>, Box<Error>> {
        let clipboard = Clipboard::new()?;
        Ok(vec![])
    }
    fn set_contents(&mut self, data: &[u8]) -> Result<(), Box<Error>> {
        Ok(())
    }
}

#[test]
fn test_bap() {
    let clipboard = Clipboard::new().unwrap();

    let mut buffer = [0; 1024];
    for i in (0..100000) {
        if let Ok(byte_count) = clipboard.get(i, &mut buffer) {
            println!("{} {}", i, byte_count);
        }
    }
}
