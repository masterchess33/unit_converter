mod units;

use std::io::{Error, ErrorKind};
use units::distance::{Centimeter, Kilometer, Meter, Millimeter};
use units::{Units, Unit};
use crate::units::volume::{CubicMeter, CubicMillimeter, Liter, Milliliter};

pub fn convert(value: f64, unit: &str, output_unit: &str) -> Result<String, Error> {
    let origin: Units = match unit.to_lowercase().as_str() {
        "km" => Units::Kilometer(Kilometer::new(value)),
        "m" => Units::Meter(Meter::new(value)),
        "cm" => Units::Centimeter(Centimeter::new(value)),
        "mm" => Units::Millimeter(Millimeter::new(value)),
        "m3" => Units::CubicMeter(CubicMeter::new(value)),
        "l" => Units::Liter(Liter::new(value)),
        "ml" => Units::Milliliter(Milliliter::new(value)),
        "mm3" => Units::CubicMillimeter(CubicMillimeter::new(value)),
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput,
                                  format!("The input unit {} is not available.", unit)));
        }
    };

    match origin {
        Units::Kilometer(x) => { x.convert(output_unit) }
        Units::Meter(x) => { x.convert(output_unit) }
        Units::Centimeter(x) => { x.convert(output_unit) }
        Units::Millimeter(x) => { x.convert(output_unit) }
        Units::CubicMeter(x) => {x.convert(output_unit) }
        Units::Liter(x) => {x.convert(output_unit) }
        Units::Milliliter(x) => {x.convert(output_unit) }
        Units::CubicMillimeter(x) => {x.convert(output_unit) }
    }
}