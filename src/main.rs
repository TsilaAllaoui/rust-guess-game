use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let mut life: i32 = 10;
    println!("Guess the hidden number!");
    let hidden_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("You have {} lives left!", life);
        if life <= 0 {
            println!("Game over! The hidden number was {}", hidden_number);
            return;
        }
        let mut chosen_number: String = String::new();
        println!("Enter a number!");
        std::io::stdin()
            .read_line(&mut chosen_number)
            .expect("Can't read user input");
        let chosen_number: u32 = match chosen_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match hidden_number.cmp(&chosen_number) {
            Ordering::Less => {
                println!("The hidden number is smaller than {}", chosen_number);
                life -= 1;
            }
            Ordering::Greater => {
                println!("The hidden number is greater than {}", chosen_number);
                life -= 1;
            }
            Ordering::Equal => {
                println!("Correct guess, the hidden number was {}", chosen_number);
                break;
            }
        };
    }
}
