use rand::Rng;
use std::io;

// Get the users input
fn getinput(playing: &mut bool, userlastchoice: &mut i32, isoption: &mut bool) {
    // Declare new String named input
    let mut input = String::new();

    // Read inputed text and assign it to String input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Decide what should happed based off of what the value of the String input is
    match input.to_uppercase().trim() {
        "EXIT" => {
            *playing = false;
            *isoption = true;
        }
        "ROCK" => {
            *userlastchoice = 1;
            *isoption = true;
        }
        "PAPER" => {
            *userlastchoice = 2;
            *isoption = true;
        }
        "SCISSORS" => {
            *userlastchoice = 3;
            *isoption = true;
        }
        _ => {
            println!("Error not an option");
            *isoption = false;
        }
    };
}

// Compare Bot's choice to Players choice
fn rockpaperscissors(b: &mut i8, u: &mut i32, isoption: &mut bool) {
    if *isoption {
        let mut temp = "";
        match b {
            1 => temp = "Rock",
            2 => temp = "Paper",
            3 => temp = "Scissors",
            _ => (),
        };
        println!("Bot choose: {}", temp);
        if *u == (*b).into() {
            println!("Tie!")
        };
        if *u == 1 && *b == 2 {
            println!("You lose!");
        };
        if *u == 2 && *b == 3 {
            println!("You lose!");
        };
        if *u == 3 && *b == 1 {
            println!("You lose!");
        };
        if *u == 2 && *b == 1 {
            println!("You win!");
        };
        if *u == 3 && *b == 2 {
            println!("You win!");
        };
        if *u == 1 && *b == 3 {
            println!("You win!");
        };
    }
}

// Main Function
fn main() {
    // Set variables
    let mut playing = true;
    let mut isoption = true;
    let mut userlastchoice = 0;

    // Tell player how to exit game
    println!("Type exit to leave game");

    // Run loop when the Bool playing equals true
    while playing {
        // Generate random number for bot
        let mut bot = rand::thread_rng().gen_range(1, 4);

        // Get player input
        getinput(&mut playing, &mut userlastchoice, &mut isoption);

        // Check who won the game then output it
        rockpaperscissors(&mut bot, &mut userlastchoice, &mut isoption);
    }
}
