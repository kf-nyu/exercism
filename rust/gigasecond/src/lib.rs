use time::PrimitiveDateTime as DateTime;
use std::time::Duration;
use std::ops::Add;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //todo!("What time is a gigasecond later than {start}");
    println!("{}", start);
    let giga_second = Duration::new(1000000000,0);
    start.add(giga_second)
}
