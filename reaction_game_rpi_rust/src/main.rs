use tokio::time::{self, Duration, sleep};
use std::error::Error;

use rppal::gpio::{Gpio, Trigger};

const GPIO_LED: u8 = 6;
const GPIO_BUTTON: u8 = 16;

#[tokio::main(worker_threads=3)]
async fn main() -> Result<(), Box<dyn Error>>{
    // let mut counter = 0;
    // let mut interval = time::interval(Duration::from_millis(300));
    let mut led = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut button  = Gpio::new()?.get(GPIO_BUTTON)?.into_input();
    button.set_interrupt(Trigger::RisingEdge)?;

    led.set_low();
    button.poll_interrupt(true, None)?;
    led.set_high();

    // loop{
    //     led.set_low();
    //     sleep(Duration::from_millis(100)).await;
    //     led.set_high();
    //     interval.tick().await; //First tick happens instantly
    //     println!("One Period");

    //     counter += 1;
    //     if counter > 10{
    //         break;
    //     }
    // }
    Ok(())
}

async fn _player(){

}
