use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    init();
    some_matches();
    guess_number_game();
}

fn some_matches() {
    let num = 3;

    match num {
        2 => println!("NO"),
        6..=100 => println!("NON PLUS"),
        3 => println!("OUI"),
        _ => println!("default"),
    }

    let boolean = false;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{}", binary);
}

fn init() {
    println!("What do you thing");
    let mut guess: String = String::new();
    stdin().read_line(&mut guess).expect("An error occured");
    println!("I'm thinking about {guess}");

    let x = 2;
    let y = 4;
    println!("x = {x} and y + 2 = {}", y + 2);
}

fn get_rand_num() -> u32 {
    return thread_rng().gen_range(1..100);
}

fn guess_number_game() {
    let mut count: u32 = 0;
    let secret_num = get_rand_num();
    println!("Guess the number AH");
    loop {
        count += 1;
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("An error occured");

        let guess: u32 = guess.trim().parse().expect("Must be a number");

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }
        println!("count left {}", count);
        if count == 20 {
            println!("You loose, the secret num was, {}", secret_num);
            break;
        }
    }
}
