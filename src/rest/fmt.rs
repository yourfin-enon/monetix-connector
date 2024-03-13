use chrono::{Datelike, Timelike};
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub fn into_date_string(date: DateTimeAsMicroseconds) -> String {
    let date = date.to_chrono_utc();

    format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}+00", date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second())
}

pub fn into_minor_amount(amount: f64, minor_digits: usize) -> u64 {
    let amount_str = amount.to_string();
    let splits: Vec<&str> = amount_str.split('.').collect();
    let major = splits[0];

    let result = if splits.len() == 2 {
        let digits = splits[1].chars().chain(std::iter::repeat('0'));
        let minor: String = digits.take(minor_digits).collect();

        format!("{major}{minor}")
    } else if splits.len() == 1 {
        let minor: String = "0".repeat(minor_digits);

        format!("{major}{minor}")
    } else {
        panic!("impossible");
    };

    println!("{:?}", result);

    result.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;
    use crate::rest::fmt::{into_date_string, into_minor_amount};

    #[test]
    fn minor_amount_1() {
        let source_amount = 10.1000;
        let minor_digits = 2;
        let minor_amount = into_minor_amount(source_amount, minor_digits);

        assert_eq!(minor_amount, 1010);
    }

    #[test]
    fn minor_amount_2() {
        let source_amount = 10.1097;
        let minor_digits = 2;
        let minor_amount = into_minor_amount(source_amount, minor_digits);

        assert_eq!(minor_amount, 1010);
    }

    #[test]
    fn minor_amount_3() {
        let source_amount = 10.0;
        let minor_digits = 2;
        let minor_amount = into_minor_amount(source_amount, minor_digits);

        assert_eq!(minor_amount, 1000);
    }

    #[test]
    fn into_date_string_1() {
        let date = DateTimeAsMicroseconds::new(1710181283221252);
        let date_string = into_date_string(date);

        assert_eq!(date_string, "2024-03-11T18:21:23+00");
    }
}