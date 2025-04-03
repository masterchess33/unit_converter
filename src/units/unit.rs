use std::io::Error;

pub trait Unit{

    fn new(number: f64) -> Self;

    fn convert(self, unit: &str) -> Result<String, Error>;

    fn print_table(&self) -> String;



}

