mod liter;
mod milliliter;
mod cubic_meter;
mod cubic_millimeter;

pub use liter::Liter;
pub use milliliter::Milliliter;
pub use cubic_meter::CubicMeter;
pub use cubic_millimeter::CubicMillimeter;
use crate::units::Unit;

enum Volume<T: Unit> {
    CubicMeter(CubicMeter),
    Liter(Liter),
    Milliliter(Milliliter),
    CubicMillimeter(CubicMillimeter),
    All(T)
}

impl<T: Unit> Volume<T> {

    fn eval(&self) -> String {
        match self {
            Volume::CubicMeter(x) => format!("{} m3", x),
            Volume::Liter(x) => format!("{} l", x),
            Volume::Milliliter(x) => format!("{} ml", x),
            Volume::CubicMillimeter(x) => format!("{} mm3", x),
            Volume::All(x) => {
                x.print_table()
            }
        }
    }
}