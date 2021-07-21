use crate::Redis;
use futures::StreamExt;
use polygon::ws::PolygonMessage;
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;
use rust_decimal::prelude::*;

pub struct Relay {
    consumer: StreamConsumer,
    redis: Redis,
}

impl Relay {
    pub fn new(consumer: StreamConsumer, redis: Redis) -> Self {
        Self { consumer, redis }
    }

    pub async fn run(&self) {
        tracing::info!("Starting relay");
        let mut stream = self.consumer.stream();
        while let Some(Ok(msg)) = stream.next().await {
            if let Some(payload) = msg.payload() {
                let maybe = serde_json::from_slice::<PolygonMessage>(payload);
                if let Ok(PolygonMessage::Trade(trade)) = maybe {
                    if trade.is_eligible() {
                        let key = format!("price/{}", trade.symbol);
                        let _ = self.redis.set(&key, trade.price.to_f64().unwrap()).await;
                    }
                }
            }
        }
    }
}
