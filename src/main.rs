/*
use rand::Rng;
use std::cmp::Ordering;
use std::io;
*/
use regex::Regex;

fn main() {
    secon_lesson();
    calc();
    ownership();
    use_rectangle();


}
fn ownership(){
    let s = String::from("Hello world");
    let word = first_word(&s);
    println!("{word}");
    let hello = &s[0..5];
    println!("{hello}");
    let t = first_word("Hi Hello You");
    println!("{t}");
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

fn calc(){
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

fn first_word(s: &str) -> &str {
    let word_bytes = s.as_bytes();

    for (i, &item) in word_bytes.iter().enumerate(){
        if item == b' ' {
            return  &s[..i];
        }
    }
    &s[..]
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn using_user(){
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("someone123@example.com");
    let user2 = build_user(String::from("value"), String::from("value"));
    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("value"),
        sign_in_count: user2.sign_in_count,
    };

    let user4 = User {
        email: String::from("value"),
        ..user1
    };
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.hight
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            hight: size,
        }
    }
}

fn use_rectangle(){
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale) ,
        hight: 50,
    };

    println!(
        "The area of the rectangle is {} aquare pixels.",
        rect1.area()
    );

    //println!("Rectangle {:#?}", rect1);
    dbg!(&rect1);

    let sqr = Rectangle::square(4);
    println!("Square {:#?}", sqr);
}
// https://doc.rust-lang.org/book/ch06-00-enums.html
// https://doc.rust-lang.org/cargo/guide/project-layout.html