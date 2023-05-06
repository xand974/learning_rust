use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    drop_reference();
    control();
    scope();
    read_out_of_range_index();
    compound_types();
    init();
    some_matches();
    guess_number_game();
}

fn drop_reference() {
    let mut s = String::new();
    s.push_str("Yes");
    drop(s); // no need
}

fn control() {
    let x = 3;
    let num = if x == 3 { 10 } else { 90 };
    println!("{num}");

    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    println!(" result {result}");

    let array = [1, 2, 3, 4, 5];

    for el in array {
        println!("{el}");
    }

    for el in 1..7 {
        println!("{el}");
    }
}

fn scope() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn compound_types() {
    let tuple: (i32, f32, i8) = (500, 2.0, 2);
    let (x, y, z) = tuple;
    let first_tuple = tuple.0;

    let collection: [i32; 5] = [1, 2, 3, 4, 5];
    let first = collection[0];
    println!(
        "{} , {} , {} + {} first tuple {}",
        x, y, z, first, first_tuple
    );
}

// expect an error
fn read_out_of_range_index() {
    let array = [1, 2, 3];
    let mut prompt_user = String::new();

    stdin()
        .read_line(&mut prompt_user)
        .expect_err("Cannot read");

    let prompt_user: usize = prompt_user.parse().expect("Cannot convert");

    println!("array value at 4 is {}", array[prompt_user]);
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
