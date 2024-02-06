/*
use rand::Rng;
use std::cmp::Ordering;
use std::io;
*/
use regex::Regex;

fn main() {
    secon_lesson();
    let x1: u32 = 98_222;
    let x2: u32 = 0x17FAE;
    let x3: u32 = 0o277656;
    let x4: u32 = 0b1_0111_1111_1010_1110;
    let x5 = b'A';
    print!("Dec: {}\nHex: {:#X}\nOct: {:#o}\nBin: {:#b}\nChar: {}\n", x1,x2,x3,x4,x5);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    print!("Sum: {}\ndifference: {}\nproduct: {}\nquotient: {}\ntruncated: {}\nremainder: {}\n", sum,difference,product,quotient,truncated,remainder);

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let s = "Hello world";
    
    print!("c: {}\nz: {}\nheart_eyed_cat: {}\nString: {}\n",c,z,heart_eyed_cat, s);
    let mut t = String::from("Hello you");
    t.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", t);

    let a: [i32; 5] = [11, 21, 31, 41, 51];

    for el in a {
        println!("El: {}", el);
    }

}

/*
fn first_lesson(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => { println!("You win!"); break;},
            Ordering::Greater => println!("Too big!"),
        }
    }
}
*/
/// # Examples
///
/// ```rust
/// let x = 5;
/// assert_eq!(foo, "foo");
/// ```
fn secon_lesson(){
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}

// https://doc.rust-lang.org/book/ch04-03-slices.html
// https://doc.rust-lang.org/cargo/guide/project-layout.html