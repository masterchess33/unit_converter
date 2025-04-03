use crate::units::distance::{Centimeter, Distance};
use crate::units::Unit;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use cli_table::{Cell, Style, Table};
use cli_table::format::Justify;

#[derive(Debug)]
pub struct Millimeter {
    value: f64,
}

impl Millimeter {

    pub fn to_centimeters(&self) -> Centimeter {
        Centimeter::new(self.value / 10.0)
    }

    pub fn into_centimeters(self) -> Centimeter {
        Centimeter::new(self.value / 10.0)
    }
}

impl Display for Millimeter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for Millimeter {

    fn new(number: f64) -> Self {
        Self { value: number }
    }

    fn convert(self, unit: &str) -> Result<String, Error> {
        let output: Distance<Millimeter> = match unit.to_lowercase().as_str() {
            "km" => Distance::Kilometer(self.into_centimeters().into_meters().into_kilometers()),
            "m" => Distance::Meter(self.into_centimeters().into_meters()),
            "cm" => Distance::Centimeter(self.into_centimeters()),
            "mm" => Distance::Millimeter(self),
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
                self.to_centimeters()
                    .into_meters()
                    .into_kilometers()
                    .cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Meters".cell(),
                self.to_centimeters()
                    .into_meters()
                    .cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Centimeters".cell(),
                self.to_centimeters().cell().justify(Justify::Right),
            ],
            vec!["Millimeters".cell(), self.cell().justify(Justify::Right)],
        ]
            .table()
            .title(vec!["Unit".cell().bold(true), "Number".cell().bold(true)])
            .bold(true);

        table.display().expect("Error while making table").to_string()
    }
}
