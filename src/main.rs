use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("神秘数字为{}",secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Equal =>{
                println!("猜对了！");
                break;
            }
            Ordering::Greater=>println!("big"),
            Ordering::Less=>println!("small"),
        }
    }

}
