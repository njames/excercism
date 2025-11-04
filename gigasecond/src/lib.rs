use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // let gigasec:u32 = 1_000_000_000;
    start + Duration::seconds(1_000_000_000)
}
