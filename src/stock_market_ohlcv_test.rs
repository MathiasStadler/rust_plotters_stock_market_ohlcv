// use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
// use rand::Rng;
// use rust_decimal::{
//     // prelude::{FromPrimitive, ToPrimitive},
//     prelude::FromPrimitive,
//     Decimal,
// };
// use rust_decimal_macros::dec;

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::stock_market_ohlcv::stupid;
    

    #[test]
    fn test_greet() {
        assert_eq!(stupid(), "Hello, stupid!");
    }
}
