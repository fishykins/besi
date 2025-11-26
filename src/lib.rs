#[macro_use]
mod macros;
pub mod angle;
pub mod density;
pub mod energy;
pub mod flow;
pub mod force;
pub mod length;
pub mod mass;
#[cfg(feature = "position")]
pub mod position;
pub mod power;
pub mod pressure;
pub mod temperature;
pub mod time;
pub mod velocity;
pub mod volume;

pub use paste;

pub trait BusiUnit: 'static {
    /// The scale factor to convert this unit to the base unit (meters for Length).
    const SCALE_FACTOR: f64;
}

/// Holds information for choosing and displaying a unit.
pub struct UnitDisplayInfo {
    /// The type name of the measurement this unit belongs to (e.g., `"Length"`).
    pub measurement_type_name: &'static str,
    /// The symbol for the unit, e.g., "km".
    pub symbol: &'static str,
    /// The scale factor relative to the base unit.
    pub scale_factor: f64,
}

// Create a global, distributed collection of `UnitDisplayInfo` structs.
inventory::collect!(UnitDisplayInfo);
