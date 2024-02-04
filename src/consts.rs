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
            SizeUnit::KB => value * 1024,
            SizeUnit::MB => value * 1024 * 1024,
            SizeUnit::GB => value * 1024 * 1024 * 1024,
        }
    }

    #[allow(dead_code)]
    pub fn from_bytes(&self, bytes: usize) -> String {
        let value = match self {
            SizeUnit::Bytes => bytes as f64,
            SizeUnit::KB => bytes as f64 / 1024.0,
            SizeUnit::MB => bytes as f64 / (1024.0 * 1024.0),
            SizeUnit::GB => bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        };
        format!("{:.1}", value)
    }
}