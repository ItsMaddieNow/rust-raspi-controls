use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use mcp3008::Mcp3008;

const GPIO_LED: u8 = 23;

//const SPI_CLOCK_SPEED: u32 = 1_000_000;
const XCHANNEL: u8 = 1;
const YCHANNEL: u8 = 2;
const BUTTONCHANNEL: u8 = 0;


fn main() -> Result<(), Box<dyn Error>> {

<<<<<<< HEAD
   
=======
    //let args = std::env::args().collect::<Vec<String>>();
>>>>>>> 1f93be4bd84e9b884ed0b90faa5b372fbb5b9f9c

    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());
    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop{
        println!("Button {}", mcp3008.read_adc(BUTTONCHANNEL).unwrap());
        println!("X Co-ord {}", mcp3008.read_adc(XCHANNEL).unwrap());
        println!("Y Co-ord {}", mcp3008.read_adc(YCHANNEL).unwrap());

        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }
    //Ok(())
}