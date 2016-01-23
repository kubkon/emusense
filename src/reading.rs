use ::byte_conv::{As as AsBytes};

#[derive(Debug,Clone,RustcEncodable)]
pub struct Reading {
    timestamp: u64,
    values: Vec<i16>,
}

impl Reading {
    pub fn new(ts: u64, values: &[i16]) -> Reading {
        Reading { timestamp: ts, values: Vec::from(values) }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.timestamp.as_bytes());
        for v in &self.values {
            bytes.extend(v.as_bytes());
        }
        bytes
    }
}

pub fn gen_readings<F>(
    num_readings: usize,
    num_values: usize,
    rate: usize,
    f: F) -> Vec<Reading> where
    F: Fn(f32) -> i16 {
    let mut readings = vec![];
    let time_int = ((1f64 / rate as f64) * 1e6f64) as u64;

    for i in 0..num_readings {
        let reading = Reading::new(
            i as u64 * time_int,
            &(0..num_values).map(|j| {
                f((i * num_values + j) as f32)
            }).collect::<Vec<i16>>()
        );
        readings.push(reading);
    }

    readings
}

pub fn readings_to_bytes(readings: &Vec<Reading>) -> Vec<u8> {
    let mut bytes = vec![];
    for r in readings {
        bytes.extend(r.to_bytes());
    }
    bytes
}
