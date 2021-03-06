use std::str;

pub trait Execute<T> {
    fn execute(&self) -> T;
}

struct CalcHugeStrings {

}
impl Execute<u8> for CalcHugeStrings {
    fn execute(&self) -> u8 {
        16
    }
}
// impl Execute<BigCalc> for BigCalc {
//     fn execute(&self) -> BigCalc {
//         return BigCalc{};
//     }
// }

impl CalcHugeStrings {
    fn new() -> CalcHugeStrings {
        return CalcHugeStrings {};
    }

    fn testy(&self){
        println!("it worked");
    }

    fn add(&self, left: &'static str, right: &'static str) -> i32{

        //pad the strings with 0's
        let padded_numbers = self.pad_to_same_length(left, right);
        println!("{:#?}", padded_numbers);
        let mut length = padded_numbers.0.len();
        let padded_left = padded_numbers.0;
        let padded_right = padded_numbers.1;

        let left_chars:Vec<char> = padded_left.chars().collect();
        let right_chars:Vec<char> = padded_right.chars().collect();
        let mut carry = false;
        let mut output = String::new();

        //loop through and call a function on each char
        while length > 0
        {
            println!("{}", left_chars[length - 1]);
            let carry_and_digit = self.add_two_and_carry(left_chars[length - 1], right_chars[length - 1], carry);
            output = format!("{}{}",  carry_and_digit.1, output);
            println!("Output = {}", output);
            carry = carry_and_digit.0;
            length = length - 1;
        }
        if carry {

            output = format!("{}{}",  1, output);
        }

        println!("Output = {}", output);

        return left.parse::<i32>().unwrap() + right.parse::<i32>().unwrap() ;
    }

    fn pad_to_same_length(&self, _left: &'static str, _right: &'static str ) -> (String, String) {
        let mut max_length = _left.len();
        if _left.len() < _right.len()  {
            max_length = _right.len();
        }

        let left_formatted = format!("{:0>width$}", _left, width = max_length);
        let right_formatted = format!("{:0>width$}", _right, width = max_length);
        return (String::from(left_formatted), String::from(right_formatted));
    }

    fn add_two_and_carry(&self, _left: char, _right: char, _carry: bool) -> (bool, u32) {
        let mut output = _left.to_digit(10).unwrap() + _right.to_digit(10).unwrap();
        if _carry{
            output = output + 1;
        }
        let digit = output % 10;
        return (output > 9, digit)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let sut = CalcHugeStrings::new();
        //let sut = BigCalc{};
        sut.testy();
        let output = sut.execute();
        println!("{}", output);
        let otheroutput = sut.add("3456", "7874");
        println!("{}", otheroutput);
        //assert_eq!(1, value_in_cents(penny));
    }
    #[test]
    fn add_something_test() {
        let mut play: Vec<char> = "875".chars().collect();
        println!("ok this  is {:#?}", play[1]);
        println!("{:#?}", play.reverse());
        println!("ok this  is reversed {:#?}", play[1]);
        for i in play {
            println!("fires");

            println!("{:#?}", i);
        }
    }
    #[test]
    fn add_two_and_carry_test() {
        let sut = CalcHugeStrings::new();
        let mut output = sut.add_two_and_carry('3', '6', false);
        println!("ok this  is {:#?}", output);
        output = sut.add_two_and_carry('7', '6', false);
        println!("ok this  is {:#?}", output);
        output = sut.add_two_and_carry('3', '4', true);
        println!("ok this  is {:#?}", output);
        output = sut.add_two_and_carry('7', '6', true);
        println!("ok this  is {:#?}", output);

        // let mut play: Vec<char> = "875".chars().collect();
        // println!("ok this  is {:#?}", play[1]);
        // println!("{:#?}", play.reverse());
        // println!("ok this  is reversed {:#?}", play[1]);
        // for i in play {
        //     println!("fires");
        //
        //     println!("{:#?}", i);
        // }
    }
    #[test]
    fn pad_to_same_length_test(){

        let sut = CalcHugeStrings::new();
        let mut output = sut.pad_to_same_length("11", "2223");
        println!("{:#?}", output);
        output = sut.pad_to_same_length("345111", "2223");
        println!("{:#?}", output);
        // let output_format = format!("Hello {:0>width$}!", "x", width = 5);
        // println!("{:#?}", output_format);


    }
}