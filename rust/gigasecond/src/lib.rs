use time::PrimitiveDateTime as DateTime;
use std::time::Duration;
use std::ops::Add;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::from_secs(1000000000))
}
