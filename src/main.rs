use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    run_game();
    play_again();
}

fn run_game() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..1000);
        println!("The secret numeber is: {}", secret_number);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!". red()),
            Ordering::Greater => println!("{}", "Too big!". red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

fn play_again() {
    println!("Play Again?");
    println!("Input Yes or No..");

    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Failed to read line");
    let ans = ans.trim().to_lowercase();

    if ans == "yes" {
        main();
    } else {
        return;
    }
}