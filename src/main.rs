use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Hello, world! guess your number ");
    
    guessing_game();
    

}

fn guessing_game()
{
   
    // genrate random number 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    
    loop{
    println!("Enter your guess number ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // println!("your guess  : {guess}"); 

    let guess:u32= guess.trim().parse().expect("please type number ");
    println!("you guess: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!(" too low"),
            Ordering::Greater => println!(" too greater "),
            Ordering::Equal => {
                println!(" Equal you win $$ ");
                println!("the secret number is:{secret_number}");
                break;
                },
        }

    }


}