use rppal::gpio::{Gpio, Trigger, InputPin, OutputPin};
use std::error::Error;
use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct GamePlayer{
    button: InputPin,
    red_led: OutputPin,
    green_led: OutputPin,
    pub id: u32,
    pub total: u128
}

impl GamePlayer{
    pub fn new(button: u8, red_led: u8, green_led: u8, id: u32 ) -> Result<GamePlayer, Box<dyn Error>>{
        let mut button = Gpio::new()?.get(button)?.into_input();
        button.set_interrupt(Trigger::RisingEdge)?;

        let mut red_led = Gpio::new()?.get(red_led)?.into_output();
        let mut green_led = Gpio::new()?.get(green_led)?.into_output();

        red_led.set_low();
        green_led.set_high();

        Ok(GamePlayer { button, red_led, green_led, id, total: 0 })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>{
        self.green_led.set_high(); 
        self.red_led.set_low();
        
        for _ in 0..5{
            let second = Duration::from_secs(1);
            let sleep_time = second + Duration::from_millis(rand::thread_rng().gen_range(0..501)*10);
            sleep(sleep_time);
         
            self.green_led.set_low();
            self.red_led.set_high();

            let now = SystemTime::now();

            self.button.poll_interrupt(true, None)?; //Infinite wait until the user presses the button

            self.green_led.set_high();
            self.red_led.set_low();

            
            let elapsed = now.elapsed()?;
        
            println!("Player {} pressed in: {}ms", self.id, elapsed.as_millis()); //Auto converts to millis for me :).
            self.total += elapsed.as_millis();
        }

        //I'll be polite and set these back to off...
        self.red_led.set_high();
        self.green_led.set_high();
        Ok(())
    }
}

