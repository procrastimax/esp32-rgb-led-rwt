use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::{thread, time::Duration};

mod led;

use led::{RGB8, WS2812RMT};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Turning RGB LED light on!");

    let mut rgb_led = WS2812RMT::new(8).expect("RGB LED should be creatable!");

    rgb_led.set_pixel(RGB8::new(255, 0, 0)).unwrap();
    thread::sleep(Duration::from_secs(3));

    rgb_led.set_pixel(RGB8::new(0, 255, 0)).unwrap();
    thread::sleep(Duration::from_secs(3));

    rgb_led.set_pixel(RGB8::new(0, 0, 255)).unwrap();
    thread::sleep(Duration::from_secs(3));

    rgb_led.set_pixel(RGB8::new(255, 255, 255)).unwrap();
    thread::sleep(Duration::from_secs(3));

    rgb_led.set_pixel(RGB8::new(90, 20, 200)).unwrap();
    thread::sleep(Duration::from_secs(3));

    println!("Turning RGB LED light off!");
    rgb_led.set_pixel(RGB8::new(0, 0, 0)).unwrap();
}
