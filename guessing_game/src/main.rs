use std::io; //standard input/output
use rand::Rng; //randon number generator
use std::cmp::Ordering; //compare library

fn main() {
    println!("Guess the number!"); 

    let secret_number = rand::thread_rng().gen_range(1, 101); //inclusive lower bound, exclusive upper bound, rust defaults number types to i32 unless otherwise specifiedi

    loop{
        println!("Please input your guess!");

        let mut guess = String::new(); //let statements create a variable, mut allows the var to be mutable, variables are immutable by default in rust
        //new is an associated function of the String type, basically just scoping
    
        io::stdin().read_line(&mut guess).expect("Failed to read line"); //calls the readline function of stdin, takes standard input and puts it into the mutable string guess, passes guess by reference to access the same piece of memory, & needs to be before mut to make the reference mutable
        //read_line returns an io result along with putting the standard input into the parameter
        //passed into it, the expect method hands errors

        //Here we want to convert String guess into a number type so it can be compared to
        //secret_number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, //underscore is a catachall value
        }; //rust allows us to shadow the previous variable, trim eliminates whitespace on either side of a string, colon tells ruse we are annotating the variable type, parse parses strings into numbers, since we are making this u32, the comparison will below will cause secret_number to become u32 as well

        println!("You guessed: {}", guess); //curly braces are used as variable place holders

        match guess.cmp(&secret_number) { //match matches the return from cmp to the arms inside the curly braces, then executes the code for the matching arm
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
