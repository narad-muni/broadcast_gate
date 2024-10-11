use std::{collections::HashMap, fs};

use rdkafka::{
    producer::{BaseRecord, DefaultProducerContext, NoCustomPartitioner, ThreadedProducer},
    ClientConfig,
};

use crate::{settings, types::packet::Packet};

use super::OutputTrait;

pub struct KafkaOutput {
    producer: ThreadedProducer<DefaultProducerContext, NoCustomPartitioner>,
    topic_name: String,
    partition_no: i32,
}

impl KafkaOutput {
    pub fn new() -> KafkaOutput {
        let settings = settings::get();

        // Read kakfa config to hashmap
        let kafka_config = fs::read_to_string(&settings.kafka_config_path).unwrap();
        let kafka_config: HashMap<String, String> = serde_json::from_str(&kafka_config).unwrap();

        let mut config = &mut ClientConfig::new();

        // build kafka producer with provided kakfa config
        for (key, val) in kafka_config {
            config = config.set(key, val);
        }

        // Build producer from config
        let producer: ThreadedProducer<DefaultProducerContext, NoCustomPartitioner> = config
            .set("bootstrap.servers", &settings.kafka_brokers)
            .create()
            .expect("Producer creation failed");

        KafkaOutput {
            producer,
            partition_no: settings.kafka_partition_no as i32,
            topic_name: settings.kafka_topic_name.clone(),
        }
    }
}

impl OutputTrait for KafkaOutput {
    fn write(&mut self, data: &Packet) {
        let payload = BaseRecord::to(&self.topic_name)
            .partition(self.partition_no)
            .key(&())
            .payload(&data);

        self.producer.send(payload).unwrap();
    }
}
