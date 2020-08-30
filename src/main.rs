use std::io;
use rand::Rng;

fn main() {
    println!("Hello! Welcome to HexBin, a program that quizzes you on conversions between\
     hex/binary/decimal on 4 bit numbers!");

    let mut rng = rand::thread_rng();
    let mut num;
    loop {
        num = rng.gen_range(0,16);
        println!("The number is {}", num);
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
