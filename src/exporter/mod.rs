use std::collections::HashMap;

use serde_json::{Map, Value};

pub mod healthcheck;
pub mod overview;
pub mod queue;

#[macro_export]
macro_rules! get_value {
    ($map:expr, $key:literal) => {
        $map.get($key).cloned().unwrap_or_default()
    };
}

#[macro_export]
macro_rules! set_gauge {
    ($key: literal, $name: literal, $help:literal) => {
        set_gauge_vec!($key, $name, $help, EMPTOY_LABEL)
    };
}

#[macro_export]
macro_rules! set_gauge_vec {
    ($key:literal, $name:literal, $help:literal, $args:expr) => {
        (
            $key.to_string(),
            GaugeVec::new(Opts::new($name, $help), $args).expect("Could not create gauge vec"),
        )
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn replace() {
        assert_eq!("rabbitmq1", "rabbit@rabbitmq1".replace("rabbit@", ""));
    }
}

trait RabbitReply {
    type MetricMap;
    type StatsInfo;
    fn make_map(&self) -> Self::MetricMap;
    fn make_stats_info(&self, labels: &[&str]) -> Vec<Self::StatsInfo>;
}

struct RabbitJsonReply<'a> {
    body: &'a Value,
}

impl<'a> RabbitJsonReply<'a> {
    fn from_response(body: &'a Value) -> Self {
        RabbitJsonReply { body }
    }
}

impl<'a> RabbitReply for RabbitJsonReply<'a> {
    type MetricMap = HashMap<String, f64>;

    type StatsInfo = (HashMap<String, String>, HashMap<String, f64>);

    fn make_map(&self) -> Self::MetricMap {
        let mut map = HashMap::new();
        if let Some(data) = self.body.as_object() {
            add_fields(&mut map, "".into(), &data);
        }
        map
    }

    fn make_stats_info(&self, labels: &[&str]) -> Vec<Self::StatsInfo> {
        let mut vec = vec![];
        if let Some(data) = self.body.as_array() {
            data.iter().for_each(|f| {
                let mut field = "";
                f.get("name").map(|_| field = "name");
                f.get("id").map(|_| field = "id");
                if !field.is_empty() {
                    let mut vec0 = HashMap::new();
                    let mut vec1 = HashMap::new();
                    labels.iter().for_each(|&d| {
                        vec0.insert(d.to_owned(), "".to_owned());
                        match f.get(d) {
                            Some(Value::String(n)) => {
                                vec0.insert(d.to_owned(), n.to_string());
                            }
                            Some(Value::Bool(n)) => {
                                vec0.insert(
                                    d.to_owned(),
                                    if *n { "1".to_owned() } else { "0".to_owned() },
                                );
                            }
                            _ => {}
                        }
                    });
                    if let Some(s) = f.as_object() {
                        add_fields(&mut vec1, "".into(), s);
                    }
                    vec.push((vec0, vec1));
                }
            });
        }
        vec
    }
}

fn add_fields(map: &mut HashMap<String, f64>, basename: String, source: &Map<String, Value>) {
    let mut prefix = basename.clone();
    if !prefix.is_empty() {
        prefix = basename + ".";
    }

    for (k, v) in source {
        match v {
            Value::Number(num) => {
                map.insert(format!("{}{}", prefix, k), num.as_f64().unwrap_or_default());
            }
            Value::Array(arr) => {
                map.insert(format!("{}_len", prefix.to_owned() + k), arr.len() as f64);
            }
            Value::Object(m) => {
                add_fields(map, format!("{}{}", prefix, k), m);
            }
            Value::Bool(b) => {
                map.insert(format!("{}{}", prefix, k), if *b { 1.0 } else { 0.0 });
            }
            _ => {}
        }
    }
}
