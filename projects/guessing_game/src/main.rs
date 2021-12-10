use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    println!("Guess the number");

    let sec_num=rand::thread_rng().gen_range(0..11);
    println!("Enter the number");
    
    loop{
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("Failes to readline");

        let guess:i32 = guess.trim().parse().expect("Please tpe a number");
        println!("you guessed the number {}",guess);
        match guess.cmp(&sec_num){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win !!\n");
                break;
            }
        }
    }
    println!("The secret number is {}",sec_num);
}