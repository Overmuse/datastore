use crate::db::DbPool;
use crate::error::Error;
use core::convert::TryInto;
use datastore_core::Dividend;
use iex::client::Client;
use iex::dividends::GetDividends;
use iex::Range;
use warp::reject::{custom, Rejection};

pub async fn list_dividends(db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    let values: Result<Vec<Dividend>, Error> = connection
        .query("SELECT * FROM dividends", &[])
        .await
        .map_err(Error::DbQueryError)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DbQueryError))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}

pub async fn store_dividend(dividend: Dividend, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    connection.execute(
        "INSERT INTO dividends (amount, declared_date, ex_date, record_date, payment_date, ticker) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&dividend.amount, &dividend.declared_date, &dividend.ex_date, &dividend.record_date, &dividend.payment_date, &dividend.ticker]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}

pub async fn backfill_dividends(ticker: String, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    tokio::spawn(async move {
        let query = GetDividends {
            symbol: &ticker,
            range: Range::FiveYears,
        };
        let client = Client::from_env().unwrap();
        let connection = db.get_connection().await.unwrap();
        let dividends = client.send(query).await.unwrap();
        for dividend in dividends {
            connection.execute(
                "INSERT INTO dividends (amount, declared_date, ex_date, record_date, payment_date, ticker) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&dividend.amount, &dividend.declared_date, &dividend.ex_date, &dividend.record_date, &dividend.payment_date, &ticker]
            ).await.unwrap();
        }
    });
    Ok(warp::reply::reply())
}
