mod centimeter;
mod kilometer;
mod meter;
mod millimeter;
// Re-export to avoid redundancy.
pub use centimeter::Centimeter;
pub use kilometer::Kilometer;
pub use meter::Meter;
pub use millimeter::Millimeter;
use crate::units::Unit;

enum Distance<T: Unit> {
    Kilometer(Kilometer),
    Meter(Meter),
    Centimeter(Centimeter),
    Millimeter(Millimeter),
    All(T),
}

impl<T: Unit> Distance<T> {

    fn eval(&self) -> String {
        match self {
            Distance::Kilometer(x) => format!("{} km", x),
            Distance::Meter(x) => format!("{} m", x),
            Distance::Centimeter(x) => format!("{} cm", x),
            Distance::Millimeter(x) => format!("{} mm", x),
            Distance::All(x) => {
                x.print_table()
            }
        }
    }
}
