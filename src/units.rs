pub mod distance;
pub mod volume;
mod unit;

pub use unit::Unit;
use distance::{Centimeter, Kilometer, Meter, Millimeter};
use volume::{CubicMeter, Liter};
use crate::units::volume::{CubicMillimeter, Milliliter};

pub enum Units {
    Kilometer(Kilometer),
    Meter(Meter),
    Centimeter(Centimeter),
    Millimeter(Millimeter),
    CubicMeter(CubicMeter),
    Liter(Liter),
    Milliliter(Milliliter),
    CubicMillimeter(CubicMillimeter),
}