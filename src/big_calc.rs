
struct BigCalc {

}

impl BigCalc {
    fn new(&self) -> BigCalc{
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
        sut.testy();
        //assert_eq!(1, value_in_cents(penny));
    }
}