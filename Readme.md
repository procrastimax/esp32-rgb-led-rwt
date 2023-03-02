# ESP32 RGB LED RWT Example

The used LED on my board (ESP32-C3-DevKitM-1) is a WS2812 and is controlled via RMT.
This example integrates the code from https://github.com/esp-rs/espressif-trainings/blob/main/common/lib/esp32-c3-dkc02-bsc/src/led.rs into the most recent version of esp_idf_sys (0.32.1) with only some changes to some used c_types and the addition of setting a GPIO Pin for the LED when calling ::new().

The default GPIO Pin for the ESP32-C3-DevKitM-1 is Pin 8 (see src/main.rs).


## How to run
`cargo espflash /dev/ttyUSB0 --speed 921600 --monitor --release`
