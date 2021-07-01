use crate::{OutputFormat, Resource};
use anyhow::Result;
use datastore_client::{Client, GetAggregates, GetDividends, GetSplits};
use prettytable::Table;

pub async fn get_resource(
    client: Client<'_>,
    resource: Resource,
    format: OutputFormat,
) -> Result<()> {
    match resource {
        Resource::Aggregates => {
            let data = client.send(GetAggregates {}).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "open", "high", "low", "close", "volume", "start", "end", "ticker",
                    ]);
                    for agg in data {
                        table.add_row(row![
                            agg.open, agg.high, agg.low, agg.close, agg.volume, agg.start, agg.end,
                            agg.ticker,
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
            let data = client.send(GetDividends {}).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "amount",
                        "declared_date",
                        "ex_date",
                        "record_date",
                        "payment_date",
                        "ticker",
                    ]);
                    for dividend in data {
                        table.add_row(row![
                            dividend.amount,
                            dividend.declared_date,
                            dividend.ex_date,
                            dividend.record_date,
                            dividend.payment_date,
                            dividend.ticker
                        ]);
                    }
                    table.printstd();
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&data)?)
                }
            }
        }
        Resource::Splits => {
            let data = client.send(GetSplits {}).await?;
            match format {
                OutputFormat::Table => {
                    let mut table = Table::new();
                    table.add_row(row![
                        "ratio",
                        "declared_date",
                        "ex_date",
                        "ticker",
                        "from_factor",
                        "to_factor"
                    ]);
                    for split in data {
                        table.add_row(row![
                            split.ratio,
                            split.declared_date,
                            split.ex_date,
                            split.ticker,
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
