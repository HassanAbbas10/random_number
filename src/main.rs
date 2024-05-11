use std::io;
use rand::Rng;

fn main(){

    println!("Enter the number you want");
    let mut user_num = String::new();

    io::stdin()
    .read_line(&mut user_num)
    .expect("Input error has been done");

    let user_num = user_num.trim().parse().expect("This is not good");
        println!("This is the user number{user_num}");


    let sec_num = rand::thread_rng().gen_range(1..=100);
        println!("This is the seret number {sec_num}");

        if sec_num == user_num
        {
            println!("You have won the game");
        }
        else{
            println!("You have lost the game")
        }

}