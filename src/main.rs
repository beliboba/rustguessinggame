use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!");
    loop {
        println!("Введи число");
        
        let secret_num = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Неправильный ввод");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ты ввёл {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Меньше!"),
            Ordering::Equal =>{
                println!("В яблочко!");
                break;
            },
            Ordering::Greater => println!("Больше!"),
        }
    }
}
