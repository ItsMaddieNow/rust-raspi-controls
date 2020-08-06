//use std::error::Error;
//use std::thread;
//use std::time::Duration;
use std::str::FromStr;

use mcp3xxx::{Channel, Mcp3008};

//use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode/*, Segment*/, SlaveSelect, Spi};

const SPI_CLOCK_SPEED: u32 = 2000000;
const XCHANNEL: u8 = 1;
const YCHANNEL: u8 = 2;
const BUTTONCHANNEL: u8 = 0;


fn main() {

    let mut mcp3008 = Mcp3008::new(Spi::new(Bus::Spi0, SlaveSelect::Ss0, SPI_CLOCK_SPEED, Mode::Mode0).unwrap()).unwrap();

    let args = std::env::args().collect::<Vec<String>>();

    let result = mcp3008.single_ended_read(Channel::new(XCHANNEL).unwrap()).unwrap();
    
    println!(
        "{}/{} = {}",
        result.value(),
        result.range(),
        result.as_fraction()
    );
    //let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    /*loop{
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    }*/
    //Ok(())
}