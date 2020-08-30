use std::io;
use rand::Rng;

fn main() {
    println!("Hello! Welcome to HexBin, a program that quizzes you on conversions between\
     hex/binary/decimal on 4 bit numbers!");

    let mut rng = rand::thread_rng();
    let mut hex_to_bin;
    let mut num;
    let mut input = String::new();
    let mut parse_guess;
    loop {
        num = rng.gen_range(0,16);
        hex_to_bin = rng.gen_bool(0.5);
        match hex_to_bin {
            true => println!("The number is {:04b} in binary, \
            convert to hex in the form X.", num),

            false => println!("The number is {:01X} in hex, \
            convert to binary in the form XXXX.", num),
        }

        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered {}", input);

        match hex_to_bin {
            false => {
                parse_guess = i32::from_str_radix(&input, 2);
            }
            true => {
                parse_guess = i32::from_str_radix(&input, 16);
            }
        }

        match parse_guess {
            Ok(guess_num) => {
                if guess_num == num {
                    println!("Yay!");
                } else {
                    println!("Booooo!");
                }
            }
            Err(error) => panic!("oopers"),
        }


        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let mut rng = rand::thread_rng();
        let mut num;
        for _ in 0..1000 {
            num = rng.gen_range(0,16);
            if (num < 0) | (num > 15) {
                panic!("Bad number");
            }
        }
    }
}
