#[allow(dead_code)]
pub enum SizeUnit {
    Bytes,
    KB,
    MB,
    GB,
}

impl SizeUnit {
    pub fn to_bytes(&self, value: usize) -> usize {
        match self {
            SizeUnit::Bytes => value,
            SizeUnit::KB => value * (1 << 10),
            SizeUnit::MB => value * (1 << 20),
            SizeUnit::GB => value * (1 << 30),
        }
    }

    #[allow(dead_code)]
    pub fn from_bytes(&self, bytes: usize) -> String {
        let value = match self {
            SizeUnit::Bytes => bytes as f64,
            SizeUnit::KB => bytes as f64 / (1 << 10) as f64,
            SizeUnit::MB => bytes as f64 / (1 << 20) as f64,
            SizeUnit::GB => bytes as f64 / (1 << 30) as f64,
        };
        format!("{:.1}", value)
    }
}