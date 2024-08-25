// run inside with - /home/user/workspace_rust/rust_fin_rs
// rm -rf out.txt && cargo test --package rust_fin_rs s_m_ohlcv::stock_market_test_ohlcv_show_chart    >out.txt 2>&1

// rm -rf out.txt && cargo test --package rust_plotters_stock_market_ohlcv >out.txt 2>&1

// FROM HERE
// https://learn-with-tests.github.io/learn-rust-with-tests/

fn greet() -> String {
    String::from("Hello, World!")
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, world!");
    }

}