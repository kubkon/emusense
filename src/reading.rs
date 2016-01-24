use std::mem::transmute;

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
        let mut bytes: Vec<u8> = vec![];
        let ts_bytes = unsafe { transmute::<u64, [u8; 8]>(self.timestamp) };
        bytes.extend(&ts_bytes);
        for v in &self.values {
            let v_bytes = unsafe { transmute::<i16, [u8; 2]>(*v) };
            bytes.extend(&v_bytes);
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

#[test]
fn test_reading_to_bytes() {
    let reading = Reading::new(
        100,
        &[-32768, 32767, 0]
    );
    let bytes = reading.to_bytes();
    assert!(bytes == vec![
        100u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        0u8, 128u8, 255u8, 127u8, 0u8, 0u8]);
}
