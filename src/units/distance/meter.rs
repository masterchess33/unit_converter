use crate::units::distance::{Centimeter, Distance, Kilometer};
use crate::units::unit::Unit;
use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Meter {
    value: f64,
}

impl Meter {
    pub fn to_centimeters(&self) -> Centimeter {
        Centimeter::new(self.value * 100.0)
    }

    pub fn to_kilometers(&self) -> Kilometer {
        Kilometer::new(self.value / 1000.0)
    }

    pub fn into_centimeters(self) -> Centimeter {
        Centimeter::new(self.value * 100.0)
    }

    pub fn into_kilometers(self) -> Kilometer {
        Kilometer::new(self.value / 1000.0)
    }
}

impl Display for Meter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for Meter {
    fn new(number: f64) -> Self {
        Self { value: number }
    }

    fn convert(self, unit: &str) -> Result<String, Error> {
        let output: Distance<Meter> = match unit.to_lowercase().as_str() {
            "km" => Distance::Kilometer(self.into_kilometers()),
            "m" => Distance::Meter(self),
            "cm" => Distance::Centimeter(self.into_centimeters()),
            "mm" => Distance::Millimeter(self.into_centimeters().into_millimeters()),
            "all" => Distance::All(self),
            _ => {
                return Err(Error::new(ErrorKind::InvalidInput,
                                      format!("The output unit {} is not available.", unit)));
            }
        };

        Ok(output.eval())
    }

    fn print_table(&self) -> String{
        let table = vec![
            vec![
                "Kilometers".cell(),
                self.to_kilometers().cell().justify(Justify::Right),
            ],
            vec!["Meters".cell(), self.cell().justify(Justify::Right)],
            vec![
                "Centimeters".cell(),
                self.to_centimeters().cell().justify(Justify::Right),
            ],
            vec![
                "Millimeters".cell(),
                self.to_centimeters()
                    .into_millimeters()
                    .cell()
                    .justify(Justify::Right),
            ],
        ]
        .table()
        .title(vec!["Unit".cell().bold(true), "Number".cell().bold(true)])
        .bold(true);

        table.display().expect("Error while making table").to_string()
    }
}
