use crate::{OutputFormat, Resource};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use datastore_client::{Client, GetAggregates, GetDividends, GetLast, GetSplits};
use prettytable::Table;

pub async fn get_resource(
    client: Client<'_>,
    resource: Resource,
    ticker: Option<String>,
    dates: Option<(NaiveDate, NaiveDate)>,
    format: OutputFormat,
) -> Result<()> {
    match resource {
        Resource::Aggregates => {
            let data = client.send(GetAggregates { ticker, dates }).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "datetime", "ticker", "open", "high", "low", "close", "volume",
                    ]);
                    for agg in data {
                        table.add_row(row![
                            agg.datetime,
                            agg.ticker,
                            agg.open,
                            agg.high,
                            agg.low,
                            agg.close,
                            agg.volume,
                        ]);
                    }
                    table.printstd();
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&data)?)
                }
            }
        }
        Resource::Dividends => {
            let data = client.send(GetDividends { ticker, dates }).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "ticker",
                        "amount",
                        "declared_date",
                        "ex_date",
                        "record_date",
                        "payment_date",
                    ]);
                    for dividend in data {
                        table.add_row(row![
                            dividend.ticker,
                            dividend.amount,
                            dividend.declared_date,
                            dividend.ex_date,
                            dividend.record_date,
                            dividend.payment_date,
                        ]);
                    }
                    table.printstd();
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&data)?)
                }
            }
        }
        Resource::Last => {
            let ticker = ticker.ok_or(anyhow!("Missing ticker"))?;
            let data = client.send(GetLast { ticker }).await?;
            println!("{:?}", data);
        }
        Resource::Splits => {
            let data = client.send(GetSplits { ticker, dates }).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "ticker",
                        "ratio",
                        "declared_date",
                        "ex_date",
                        "from_factor",
                        "to_factor"
                    ]);
                    for split in data {
                        table.add_row(row![
                            split.ticker,
                            split.ratio,
                            split.declared_date,
                            split.ex_date,
                            split.from_factor,
                            split.to_factor
                        ]);
                    }
                    table.printstd();
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&data)?)
                }
            }
        }
    }
    Ok(())
}
