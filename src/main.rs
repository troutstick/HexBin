use std::io;
use rand::Rng;

fn main() {
    println!("Hello! Welcome to HexBin, a program that quizzes you on conversions between \
    hex/binary/decimal on 4 bit numbers!\n");

    let mut rng = rand::thread_rng();
    let exit_str = "If you want to exit, enter `q`.";
    loop {
        let mut input = String::new();
        let num = rng.gen_range(0,16);
        let hex_to_bin = rng.gen_bool(0.5);

        let binary_fmt = format!("{:04b}", num);
        let hex_fmt = format!("{:01X}", num);
        let answer = if hex_to_bin {
            println!("The number is 0x{} in hex, \
            convert to binary in the form XXXX.", hex_fmt);
            binary_fmt
        } else {
            println!("The number is 0b{} in binary, \
            convert to hex in the form X.", binary_fmt);
            hex_fmt
        };

        println!("{}", exit_str);

        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered {}.", input.trim());

        let slice: &str = &input.trim();
        let parse_guess = if hex_to_bin {
            i32::from_str_radix(slice, 2)
        } else {
            i32::from_str_radix(slice, 16)
        };

        match parse_guess {
            Ok(guess_num) => {
                if guess_num == num {
                    println!("Yay! You did it!");
                } else {
                    println!("Booooo! The answer was {}!", answer);
                }
            }
            Err(_) => {
                if input.trim().eq("q") {
                    println!("Quitting...");
                    break;
                } else {
                    println!("Unable to parse input. {}", exit_str);
                }
            },
        }
        println!();
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
            if (num < 0) || (num > 15) {
                panic!("Bad number");
            }
        }
    }
}
