
struct BigCalc {

}

impl BigCalc {
    fn new() -> BigCalc{
        return BigCalc{};
    }
    fn testy(&self){
        println!("it worked");
    }
    fn add(&self, left: &'static str, right: &'static str) -> i32{
        return left.parse::<i32>().unwrap() + right.parse::<i32>().unwrap() ;
        //pad the strings with 0's
    }
    fn pad_to_same_length(&self, left: &'static str, right: &'static str ) -> (String, String) {
        return (String::from("test"), String::from("other"));
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let sut = BigCalc::new();
        //let sut = BigCalc{};
        sut.testy();
        let output = sut.add("22", "33");
        println!("{}", output);
        //assert_eq!(1, value_in_cents(penny));
    }

    #[test]
    fn pad_to_same_length_test(){

    }
}