use crate::units::volume::{CubicMeter, Milliliter, Volume};
use crate::units::Unit;
use cli_table::format::Justify;
use cli_table::{Cell, Style, Table};
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

pub struct Liter {
    value: f64,
}

impl Liter {
    pub fn to_milliliters(&self) -> Milliliter {
        Milliliter::new(self.value * 1000.0)
    }

    pub fn into_milliliters(self) -> Milliliter {
        Milliliter::new(self.value * 1000.0)
    }

    pub fn to_cubic_meters(&self) -> CubicMeter {
        CubicMeter::new(self.value / 1000.0)
    }

    pub fn into_cubic_meters(self) -> CubicMeter {
        CubicMeter::new(self.value / 1000.0)
    }
}

impl Display for Liter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for Liter {
    fn new(number: f64) -> Self {
        Self { value: number }
    }

    fn convert(self, unit: &str) -> Result<String, Error> {
        let output: Volume<Liter> = match unit.to_lowercase().as_str() {
            "m3" => Volume::CubicMeter(self.into_cubic_meters()),
            "l" => Volume::Liter(self),
            "ml" => Volume::Milliliter(self.into_milliliters()),
            "mm3" => Volume::CubicMillimeter(self.into_milliliters().into_cubic_millimeters()),
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
                self.to_cubic_meters().cell().justify(Justify::Right),
            ],
            vec!["Liter".cell(), self.cell().justify(Justify::Right)],
            vec![
                "Milliliter".cell(),
                self.to_milliliters().cell().justify(Justify::Right),
            ],
            vec![
                "Cubic millimeter".cell(),
                self.to_milliliters()
                    .into_cubic_millimeters()
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
