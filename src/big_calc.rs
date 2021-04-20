
struct BigCalc {

}

impl BigCalc {
    fn new() -> BigCalc{
        return BigCalc{};
    }
    fn testy(&self){
        println!("it worked");
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {


        let sut = BigCalc::new();
        //let sut = BigCalc{};
        sut.testy();
        //assert_eq!(1, value_in_cents(penny));
    }
}