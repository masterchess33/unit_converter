use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use cli_table::{Cell, Style, Table};
use cli_table::format::Justify;
use crate::units::distance::{Distance, Meter};
use crate::units::Unit;

#[derive(Debug)]
pub struct Kilometer {
    value: f64
}


impl Kilometer {

    pub fn to_meters(&self) -> Meter {
        Meter::new(self.value * 1000.0)
    }

    pub fn into_meters(self) -> Meter {
        Meter::new(self.value * 1000.0)
    }
}

impl Display for Kilometer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for Kilometer {

    fn new(number: f64) -> Kilometer {
        Kilometer { value: number }
    }

    fn convert(self, unit: &str) -> Result<String, Error> {
        let output: Distance<Kilometer> = match unit.to_lowercase().as_str() {
            "km" => Distance::Kilometer(self),
            "m" => Distance::Meter(self.into_meters()),
            "cm" => Distance::Centimeter(self.into_meters().into_centimeters()),
            "mm" => Distance::Millimeter(self.into_meters().into_centimeters().into_millimeters()),
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
            vec!["Kilometers".cell(), self.cell().justify(Justify::Right)],
            vec![
                "Meters".cell(),
                self.to_meters().cell().justify(Justify::Right),
            ],
            vec![
                "Centimeters".cell(),
                self.to_meters()
                    .to_centimeters()
                    .cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Millimeters".cell(),
                self.to_meters()
                    .into_centimeters()
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