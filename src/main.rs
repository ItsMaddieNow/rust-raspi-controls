use std::error::Error;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::fs::{self, File};
use std::fs::remove_file;
use std::io::prelude::*;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

use mcp3008::Mcp3008;

use input_linux::{UInputHandle, EventKind};

use nix::unistd::Uid;

use serde_derive::{Deserialize,Serialize};  

#[derive(Deserialize,Serialize)]
struct Layout {
    config_version: f64,
    pins: Pins,
}

#[derive(Deserialize,Serialize)]
struct Pins {
    a: u8,
    b: u8,
    x: u8,
    y: u8,
    dpad_up: u8,
    dpad_down: u8,
    dpad_left: u8,
    dpad_right: u8,
    l: u8,
    r: u8,
    zl: u8,
    zr: u8,
}

//const SPI_CLOCK_SPEED: u32 = 1_000_000;
const LEFT_X_CHANNEL: u8 = 1;
const LEFT_Y_CHANNEL: u8 = 2;
const LEFT_BUTTON_CHANNEL: u8 = 0;
const RIGHT_X_CHANNEL: u8 = 4;
const RIGHT_Y_CHANNEL: u8 = 5;
const RIGHT_BUTTON_CHANNEL: u8 = 3;

const CONFIG_LOCATION : &str = "/etc/Controller_Config.toml";
const CONFIG_VERSION : f64 = 0.2;

// Taken From <linux/input.h>
const BUS_USB: u16 = 0x03;
// Virtual Device Info
const VENDOR: u16 = 0x3232;
const VERSION: u16 = 0x1234;
const PRODUCT: u16 = 0x5678;

fn main() -> Result<(), Box<dyn Error>> {

    // Buttons
    let a: u8 = 4;
    let b: u8 = 6;
    let x: u8 = 0;
    let y: u8 = 0;
    let dpad_up: u8 = 0;
    let dpad_down: u8 = 0;
    let dpad_left: u8 = 0;
    let dpad_right: u8 = 0;
    let l: u8 = 0;
    let r: u8 = 0;
    let zl: u8 = 0;
    let zr: u8 = 0;

    println!("Running on a {}.", DeviceInfo::new()?.model());

    println!("Performing checks");
    if !Uid::effective().is_root() {
        panic!("Run me as root!");
    }

    if !Path::new("/dev/spidev0.0").exists() {
        panic!("Spi Isn't enabled! Enable it in the /boot/config.txt file or using the raspi-config command.");
    }
    if !Path::new(CONFIG_LOCATION).exists() {
        println!("Config file not found, creating file at {}", CONFIG_LOCATION);

        let mut config = File::create(CONFIG_LOCATION)?;
        
        let layout = Layout {
            config_version : CONFIG_VERSION,
            pins : Pins {
                a : a,
                b : b,
                x : x,
                y : y,  
                dpad_up : dpad_up,
                dpad_down : dpad_down,
                dpad_left : dpad_left,
                dpad_right : dpad_right,
                l : l,
                r : r,
                zl : zl,
                zr : zr,
            },
        };
        let toml =  toml::to_string(&layout).unwrap();
        config.write_all(toml.as_bytes()).ok();
        println!("Config file created");
    } 
    println!("Reading file..."); 
    let mut config = File::open(CONFIG_LOCATION)?;
    let mut contents = String::new();
    config.read_to_string(&mut contents)?;
    let layout : Layout = toml::from_str(&contents[..]).unwrap();

    if layout.config_version < CONFIG_VERSION {
        println!("Existing config file ({}) out of date, Updating to {}",layout.config_version, CONFIG_VERSION);
        
        remove_file(CONFIG_LOCATION)?;
        let mut config = File::create(CONFIG_LOCATION)?;

        let layout = Layout {
            config_version : CONFIG_VERSION,
            pins : Pins {
                a : a,
                b : b,
                x : x,
                y : y,  
                dpad_up : dpad_up,
                dpad_down : dpad_down,
                dpad_left : dpad_left,
                dpad_right : dpad_right,
                l : l,
                r : r,
                zl : zl,
                zr : zr,
            },
        };
        let toml = toml::to_string(&layout).unwrap();

        config.write_all(toml.as_bytes()).ok();
        println!("Updated!");
    }

    println!("A Pin = {}", layout.pins.a);

    let mut mcp3008 = Mcp3008::new("/dev/spidev0.0").unwrap();
    let gpio = Gpio::new()?;

    let pin1 = gpio.get(a)?.into_input_pullup();

    // Uinput setup
    println!("Setting Up Uinput device");
    // Uinput code stolen from https://github.com/mthadley/keyswitch

    let uinput = fs::OpenOptions::new().write(true).open("/dev/uinput")?;
    let output_device = UInputHandle::new(uinput);

    output_device.set_evbit(EventKind::Absolute)?;
    
    println!("Set-up Complete");
    
    print!("Sending Joystick data in 10 ");
    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));
        print!(".");
    }
    println!("");

    //device

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop{
        println!("Joystick 1 :");
        println!("    Button {}", mcp3008.read_adc(LEFT_BUTTON_CHANNEL).unwrap());
        println!("    X Co-ord {}", mcp3008.read_adc(LEFT_X_CHANNEL).unwrap());
        println!("    Y Co-ord {}", mcp3008.read_adc(LEFT_Y_CHANNEL).unwrap());

        println!("Joystick 2 :");
        println!("    Button {}", mcp3008.read_adc(RIGHT_BUTTON_CHANNEL).unwrap());
        println!("    X Co-ord {}", mcp3008.read_adc(RIGHT_X_CHANNEL).unwrap());
        println!("    Y Co-ord {}", mcp3008.read_adc(RIGHT_Y_CHANNEL).unwrap());

        println!("Button A : {}", pin1.read());

        thread::sleep(Duration::from_secs(1));
    }
    //Ok(())
}