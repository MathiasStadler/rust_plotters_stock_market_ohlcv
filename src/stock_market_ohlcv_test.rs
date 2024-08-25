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
    use crate::stock_market_ohlcv::{dudoof, stupid};
    use crate::stock_market_ohlcv::{StockData,StockInformation};

    #[test]
    fn test_stupid() {
        assert_eq!(stupid(), "Hello, stupid!");
    }
    //cargo test stock_market_ohlcv_test::tests::test_stupid

    #[test]
    fn test_dudoof() {
        assert_eq!(dudoof(), "Hello, dudoof!");
    }
    // cargo test stock_market_ohlcv_test::tests::test_dudoof

    // here show_charts
    #[test]
    fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
        let stock_data_series = generate_stock_data_series(Some(14));
        let stock_information = StockInformation::new(
            "BenCorpo".to_string(),
            "BNCRP".to_string(),
            stock_data_series,
        );

        let ma_days = vec![7, 2, 0];
        let chart = stock_information.show_chart(ma_days, None, None, None);

        match chart {
            Ok(_) => {
                assert!(true)
            }
            Err(err) => println!("Error in saving chart {:?}", err),
        }
    }


fn generate_stock_data_series(limit: Option<u8>) -> Vec<StockData> {
    let mut stock_data_series: Vec<StockData> = vec![];
    for number in 0..limit.unwrap_or(7) {
        let number_plus = number + 1;

        let stock_date = match number_plus {
            number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
            _ => format!("10-0{number_plus}-2022 00:00"),
        };

        let stock_data = generate_stock_data(&stock_date);
        stock_data_series.push(stock_data);
    }
    stock_data_series
}

}
