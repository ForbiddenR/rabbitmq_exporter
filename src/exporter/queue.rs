use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};

use crate::{client::request, config::Conf, get_gauge_vec, query, response::queue::QueueResposne};

const ENDPOINT: &str = "queues";

struct QueueMetric {
    gauge_vec: GaugeVec,
    value_fn: fn(&QueueResposne) -> f64,
}

impl QueueMetric {
    fn new(gauge_vec: GaugeVec, value_fn: fn(&QueueResposne) -> f64) -> Self {
        Self {
            gauge_vec,
            value_fn,
        }
    }
}

pub struct QueueExporter {
    queue_gauge_vec: Vec<QueueMetric>,
}

impl QueueExporter {
    pub fn new() -> Self {
        let queue_labels = ["queue"];
        let queue_gauge_vec = vec![QueueMetric::new(
            get_gauge_vec!(
                "queue_messages",
                "Sum of ready and unacknowledged messages (queue depth).",
                &queue_labels
            ),
            |q| q.messages as f64,
        )];
        QueueExporter { queue_gauge_vec }
    }

    pub async fn collect(&self, config: &Conf) -> Result<Vec<MetricFamily>> {
        self.queue_gauge_vec
            .iter()
            .for_each(|f| f.gauge_vec.reset());

        let resp: Vec<QueueResposne> = query!(config);

        for q in &resp {
            self.queue_gauge_vec.iter().for_each(|f| {
                f.gauge_vec
                    .with_label_values(&[&q.name])
                    .set((f.value_fn)(q))
            });
        }

        Ok(self
            .queue_gauge_vec
            .iter()
            .flat_map(|f| f.gauge_vec.collect())
            .collect())

        // let mut metricfamiles = self
        //     .queue_gauge_vec
        //     .iter()
        //     .flat_map(|f| f.gauge_vec.collect())
        //     .collect::<Vec<MetricFamily>>();

        // for i in &mut metricfamiles {
        //     i.metric.sort_by(|x, y| {
        //         let mut ordering = y
        //             .gauge
        //             .value()
        //             .partial_cmp(&x.gauge.value())
        //             .unwrap_or(std::cmp::Ordering::Equal);

        //         if ordering == std::cmp::Ordering::Equal {
        //             ordering = x
        //                 .get_label()
        //                 .get(0)
        //                 .unwrap_or_default()
        //                 .value()
        //                 .cmp(&y.get_label().get(0).unwrap_or_default().value());
        //         }
        //         ordering
        //     });
        // }
        // Ok(metricfamiles)
    }
}
