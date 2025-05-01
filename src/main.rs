#![no_std]
#![no_main]

use panic_halt as _;

use dht11::Dht11;
use avr_device::interrupt;
use core::cell::RefCell;
use pcd8544_hal::Pcd8544Gpio;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Initialize the serial console
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(arduino_hal::default_serial!(dp, pins, 57600));
    });

    let mut delay = arduino_hal::Delay::new();
    let mut dht11 = Dht11::new(pins.d2.into_opendrain());

    loop {
        // dht11::Error doesn't implement uDisplay
        if let Ok(dht11::Measurement { temperature: temp, humidity: hum } ) =  dht11.perform_measurement(&mut delay) {
            println!("Temperature: {}°C, Humidity: {}%", temp / 10, hum / 10);
        }
        arduino_hal::delay_ms(1000);
    }
}
