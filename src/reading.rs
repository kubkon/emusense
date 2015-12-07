use ::rand::distributions::{IndependentSample, Range};
use ::byte_conv::{As as AsBytes};

#[derive(Debug,Clone,RustcEncodable)]
pub struct Reading {
    timestamp: u64,
    x: i16,
    y: i16,
    z: i16,
}

impl Reading {
    pub fn new(ts: u64, x: i16, y: i16, z: i16) -> Reading {
        Reading { timestamp: ts, x: x, y: y, z: z }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.timestamp.as_bytes());
        bytes.extend(self.x.as_bytes());
        bytes.extend(self.y.as_bytes());
        bytes.extend(self.z.as_bytes());
        bytes
    }
}

pub fn gen_readings(num: usize) -> Vec<Reading> {
    let mut readings = vec![];
    let range = Range::new(-32768i16, 32767i16);
    let mut rng = ::rand::thread_rng();

    for _ in 0..num {
        let reading = Reading::new(
            ::time::precise_time_ns(),
            range.ind_sample(&mut rng),
            range.ind_sample(&mut rng),
            range.ind_sample(&mut rng)
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
