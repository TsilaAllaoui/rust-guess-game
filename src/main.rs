use rand::Rng;

fn main() {
    let mut life: i32 = 3;
    println!("Guess the hidden number!");
    let hidden_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut chosen_number: u32 = 0;

    while chosen_number != hidden_number {
        if life <= 0 {
            break;
        }
        println!("Enter a number!");
        let mut s: String = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .expect("Can't read user input");
        chosen_number = s.trim().parse().expect("Not a valid number!");
        if chosen_number < hidden_number {
            println!("The hidden number is bigger than {}", chosen_number);
            if life > 0 {
                life -= 1;
            }
        } else if chosen_number > hidden_number {
            println!("The hidden number is smaller than {}", chosen_number);
            if life > 0 {
                life -= 1;
            }
        }
    }

    if life <= 0 {
        println!("Game over! The hidden number was {}", hidden_number)
    }
    println!("You found the hidden number that was {}!", hidden_number);
}
