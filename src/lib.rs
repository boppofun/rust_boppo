#![doc = include_str!("../README.md")]

use boppo_core::{color, executor, Buttons};

pub async fn press_for_fun() -> ! {
    loop {
        Buttons::all().set_color(color::OFF);
        Buttons::currently_pressed().set_color(color::PURPLE);
        executor::sleep_ms(10).await;
    }
}
