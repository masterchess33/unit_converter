use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use cli_table::{Cell, Style, Table};
use cli_table::format::Justify;
use crate::units::Unit;
use crate::units::volume::{Liter, Volume};

pub struct CubicMeter {
    value: f64,
}

impl CubicMeter {

    pub fn to_liters(&self) -> Liter {
        Liter::new(self.value * 1000.0)
    }

    pub fn into_liters(self) -> Liter {
        Liter::new(self.value * 1000.0)
    }

}

impl Display for CubicMeter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Unit for CubicMeter {
    fn new(number: f64) -> Self {
        Self { value: number }
    }

    fn convert(self, unit: &str) -> Result<String,Error> {

        let output: Volume<CubicMeter> = match unit.to_lowercase().as_str() {
            "m3" => Volume::CubicMeter(self),
            "l" => Volume::Liter(self.into_liters()),
            "ml" => Volume::Milliliter(self.into_liters().into_milliliters()),
            "mm3" => Volume::CubicMillimeter(self.into_liters().into_milliliters().into_cubic_millimeters()),
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
                self.cell()
                    .justify(Justify::Right),
            ],
            vec![
                "Liter".cell(),
                self.to_liters().cell().justify(Justify::Right),
            ],
            vec!["Milliliter".cell(), self.to_liters().into_milliliters().cell().justify(Justify::Right)],
            vec![
                "Cubic millimeter".cell(),
                self.to_liters().into_milliliters().into_cubic_millimeters().cell().justify(Justify::Right),
            ],
        ]
            .table()
            .title(vec!["Unit".cell().bold(true), "Number".cell().bold(true)])
            .bold(true);

        table.display().expect("Error while making table").to_string()
    }
}