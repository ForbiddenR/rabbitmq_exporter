use std::{collections::HashMap, time::Duration};

use prometheus::{GaugeVec, register_gauge_vec};
use reqwest::{Error, Response};
use serde_json::{Map, Value};

use crate::config::Config;

pub mod overview;
pub mod queue;

#[macro_export]
macro_rules! set_field {
    ($node_info:expr, $value:expr, $field:literal, $target: ident) => {
        if let Some(val) = $value.get($field).and_then(|f| f.as_str()) {
            $node_info.$target = val.into();
        }
    };
}

#[macro_export]
macro_rules! get_value {
    ($map:expr, $key:literal) => {
        $map.get($key).cloned().unwrap_or_default()
    };
}

#[macro_export]
macro_rules! set_gauge_vec {
    ($key:literal, $name:literal, $help:literal, $args:expr) => {
        ($key.to_string(), new_gauge_vec($name, $help, $args))
    };
}

pub fn new_gauge_vec(name: &str, help: &str, tags: &[&str]) -> GaugeVec {
    register_gauge_vec!(name, help, tags).expect("Could not create gauge")
}

pub fn make_status_info(
    v: Value,
    labels: &[&str],
) -> Vec<(HashMap<String, String>, HashMap<String, f64>)> {
    let mut vec = vec![];
    if let Some(data) = v.as_array() {
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

pub fn parse_value(v: Value) -> HashMap<String, f64> {
    let mut map = HashMap::new();
    if let Some(data) = v.as_object() {
        add_fields(&mut map, "".into(), &data);
    }
    map
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

async fn request(config: &Config, endpoint: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/{}/{}", config.rabbit_url, "api", endpoint))
        .basic_auth(
            config.rabbit_user.to_string(),
            Some(config.rabbit_pass.to_string()),
        )
        .timeout(Duration::from_secs(config.timeout as u64))
        .send()
        .await
}
