#![allow(deprecated)]
extern crate ws2811;

use ws2811::*;
use std::mem;

fn main() {
  println!("Starting");
  unsafe {
      let mut ledstring = ws2811_t {
        device: mem::uninitialized(),
        rpi_hw: mem::uninitialized(),
        freq: 800000,
        dmanum: 5,
        channel: [ws2811_channel_t {
                    gpionum: 18,
                    invert: 0,
                    count: 12,
                    brightness: 32,
                    strip_type: mem::uninitialized(),
                    leds: mem::uninitialized(),
                }, 
                  ws2811_channel_t {
                    gpionum: 0,
                    invert: 0,
                    count: 0,
                    brightness: 0,
                    strip_type: mem::uninitialized(),
                    leds: mem::uninitialized(),
                }]
      };
      ws2811_init(&mut ledstring);                          // initialize
      
      set_led(&mut ledstring, 1, ws2811::DOT_COLORS[0]);    // set LED 1 to color RED(?)
      ws2811_render(&mut ledstring);                        // update hardware
      
      std::thread::sleep_ms(1000);
      ws2811_fini(&mut ledstring);                          // Finish before exiting program
   }
}
