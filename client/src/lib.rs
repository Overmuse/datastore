mod client;
mod error;
mod request;
mod routes;

pub use client::Client;
pub use error::Error;
pub use request::Request;
pub use routes::*;

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    #[tokio::test]
    async fn post_split() {
        let client = Client::from_env().unwrap();
        let split = datastore_core::Split::new(
            Decimal::new(4, 1),
            NaiveDate::from_ymd(2021, 1, 1),
            NaiveDate::from_ymd(2021, 1, 1),
            "GOOG".to_string(),
            Decimal::new(4, 0),
            Decimal::new(1, 0),
        );
        println!("{:?}", client.send(PostSplit(split)).await.unwrap());
    }
}
