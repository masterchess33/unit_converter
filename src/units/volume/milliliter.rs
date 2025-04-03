use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use cli_table::{Cell, Style, Table};
use cli_table::format::Justify;
use crate::units::Unit;
use crate::units::volume::{CubicMillimeter, Liter, Volume};

pub struct Milliliter {
    value: f64,
}

impl Milliliter {

    pub fn to_liters(&self) -> Liter {
        Liter::new(self.value / 1000.0)
    }

    pub fn into_liters(self) -> Liter {
        Liter::new(self.value / 1000.0)
    }

    pub fn to_cubic_millimeters(&self) -> CubicMillimeter {
        CubicMillimeter::new(self.value * 1000.0)
    }

    pub fn into_cubic_millimeters(self) -> CubicMillimeter {
        CubicMillimeter::new(self.value * 1000.0)
    }

}

impl Display for Milliliter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for Milliliter {
    fn new(number: f64) -> Self {
        Self { value: number }
    }

    fn convert(self, unit: &str) -> Result<String, Error> {
        let output: Volume<Milliliter> = match unit.to_lowercase().as_str() {
            "m3" => Volume::CubicMeter(self.into_liters().into_cubic_meters()),
            "l" => Volume::Liter(self.into_liters()),
            "ml" => Volume::Milliliter(self),
            "mm3" => Volume::CubicMillimeter(self.into_cubic_millimeters()),
            "all" => Volume::All(self),
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
                "Cubic meter".cell(),
                self.to_liters().into_cubic_meters().cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Liter".cell(),
                self.to_liters().cell().justify(Justify::Right),
            ],
            vec!["Milliliter".cell(), self.cell().justify(Justify::Right)],
            vec![
                "Cubic millimeter".cell(),
                self.to_cubic_millimeters().cell().justify(Justify::Right),
            ],
        ]
            .table()
            .title(vec!["Unit".cell().bold(true), "Number".cell().bold(true)])
            .bold(true);

        table.display().expect("Error while making table").to_string()
    }
}