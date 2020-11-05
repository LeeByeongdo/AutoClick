use enigo::*;
use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new();

    loop {
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(1000));
    }

}
