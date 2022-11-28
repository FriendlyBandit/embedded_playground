use std::error::Error;
use std::thread;

//Import our other file
mod gameplayer;

//Player 1 constants
const P1_LED_GREEN: u8 = 6;
const P1_LED_RED: u8 = 13;
const P1_BUTTON: u8 = 16;

//Player 2 constants
const P2_LED_GREEN: u8 = 26;
const P2_LED_RED: u8 = 20;
const P2_BUTTON: u8 = 19;

#[tokio::main(worker_threads=3)]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut player1 = gameplayer::GamePlayer::new(P1_BUTTON, P1_LED_RED, P1_LED_GREEN, 1)?;
    let mut player2 = gameplayer::GamePlayer::new(P2_BUTTON, P2_LED_RED, P2_LED_GREEN, 2)?;

    let p_1_thread = thread::spawn(move || -> Result<gameplayer::GamePlayer, Box<dyn Error + Send + Sync>> {
        player1.run()?;

        Ok(player1)
    });

   let p_2_thread = thread::spawn(move || -> Result<gameplayer::GamePlayer, Box<dyn Error + Send + Sync>> {
        player2.run()?;

        Ok(player2)
    });

    player1 = p_1_thread.join().unwrap().unwrap(); 
    player2 = p_2_thread.join().unwrap().unwrap(); 
    
    println!("Scores for the game\n Player {}: {}\n Player {}: {}", player1.id, player1.total, player2.id, player2.total);
    Ok(())
}
