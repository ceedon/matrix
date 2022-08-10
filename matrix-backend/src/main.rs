use adafruit_led_backpack::*;
use ht16k33::{Display, HT16K33};
use rppal::i2c::I2c;

fn _clear(mut matrix: HT16K33<I2c>) {

matrix
    .set_display(Display::OFF)
    .expect("could not switch the display off");
}
fn main() {
    let i2c = I2c::new()
        .expect("could not initialize I2c on your RPi, is the interface enabled in raspi-config?");
    let mut ht16k33 = HT16K33::new(i2c, 0x70);
    ht16k33.initialize().expect("failed to initialize HT16K33");
    ht16k33
        .set_display(Display::ON)
        .expect("could not switch the display on");

    let (x,y) = (0, 0);
    let color = Color::Green; 
    ht16k33
        .update_bicolor_led(x, y, color)
        .expect("failed to update LED");
    println!("Done!");
}
