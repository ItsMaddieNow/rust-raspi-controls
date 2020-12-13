use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use mcp3008::Mcp3008;

const GPIO_LED: u8 = 23;

//const SPI_CLOCK_SPEED: u32 = 1_000_000;
const LEFT_X_CHANNEL: u8 = 1;
const LEFT_Y_CHANNEL: u8 = 2;
const LEFT_BUTTON_CHANNEL: u8 = 0;
const RIGHT_X_CHANNEL: u8 = 4;
const RIGHT_Y_CHANNEL: u8 = 5;
const RIGHT_BUTTON_CHANNEL: u8 = 3;


fn main() -> Result<(), Box<dyn Error>> {


    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop{
        println!("Joystick 1 :");
        println!("    Button {}", mcp3008.read_adc(BUTTON_CHANNEL).unwrap());
        println!("    X Co-ord {}", mcp3008.read_adc(XCHANNEL).unwrap());
        println!("    Y Co-ord {}", mcp3008.read_adc(YCHANNEL).unwrap());

        println!("Joystick 2 :");
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }
    //Ok(())
}