use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Storage {
    pub quotient: u64,
    pub remainder: u64,
    pub unit: Unit,
}

impl Serialize for Storage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StorageSerde {
            quotient: u64,
            remainder: u64,
            /// Matches frontend `Unit` string literals (`"KB"`, …).
            unit: String,
            bytes: u64,
            display: String,
        }
        StorageSerde {
            quotient: self.quotient,
            remainder: self.remainder,
            unit: self.unit.to_string(),
            bytes: self.to_bytes(),
            display: format!("{:.2} {}", self.to_float(), self.unit),
        }
        .serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Unit::B => "B",
                Unit::KB => "KB",
                Unit::MB => "MB",
                Unit::GB => "GB",
                Unit::TB => "TB",
                Unit::PB => "PB",
            }
        )
    }
}

impl Storage {
    const SHIFT_KB: u64 = 10;
    const SHIFT_MB: u64 = 20;
    const SHIFT_GB: u64 = 30;
    const SHIFT_TB: u64 = 40;
    const SHIFT_PB: u64 = 50;

    pub fn new(quotient: u64, remainder: u64, unit: Unit) -> Self {
        Self {
            quotient,
            remainder,
            unit,
        }
    }

    pub fn from_bytes(bytes: u64) -> Self {
        if bytes >= (1 << Self::SHIFT_PB) {
            let q = bytes >> Self::SHIFT_PB;
            let r = bytes & ((1 << Self::SHIFT_PB) - 1);
            Storage::new(q, r, Unit::PB)
        } else if bytes >= (1 << Self::SHIFT_TB) {
            let q = bytes >> Self::SHIFT_TB;
            let r = bytes & ((1 << Self::SHIFT_TB) - 1);
            Storage::new(q, r, Unit::TB)
        } else if bytes >= (1 << Self::SHIFT_GB) {
            let q = bytes >> Self::SHIFT_GB;
            let r = bytes & ((1 << Self::SHIFT_GB) - 1);
            Storage::new(q, r, Unit::GB)
        } else if bytes >= (1 << Self::SHIFT_MB) {
            let q = bytes >> Self::SHIFT_MB;
            let r = bytes & ((1 << Self::SHIFT_MB) - 1);
            Storage::new(q, r, Unit::MB)
        } else if bytes >= (1 << Self::SHIFT_KB) {
            let q = bytes >> Self::SHIFT_KB;
            let r = bytes & ((1 << Self::SHIFT_KB) - 1);
            Storage::new(q, r, Unit::KB)
        } else {
            Storage::new(bytes, 0, Unit::B)
        }
    }

    pub fn to_bytes(&self) -> u64 {
        match self.unit {
            Unit::B => self.quotient,
            Unit::KB => (self.quotient << Self::SHIFT_KB) | self.remainder,
            Unit::MB => (self.quotient << Self::SHIFT_MB) | self.remainder,
            Unit::GB => (self.quotient << Self::SHIFT_GB) | self.remainder,
            Unit::TB => (self.quotient << Self::SHIFT_TB) | self.remainder,
            Unit::PB => (self.quotient << Self::SHIFT_PB) | self.remainder,
        }
    }

    pub fn to_float(&self) -> f64 {
        const SCALE_KB: f64 = 1.0 / 1024.0;
        const SCALE_MB: f64 = 1.0 / (1024.0 * 1024.0);
        const SCALE_GB: f64 = 1.0 / (1024.0 * 1024.0 * 1024.0);
        const SCALE_TB: f64 = 1.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0);
        const SCALE_PB: f64 = 1.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0);

        match self.unit {
            Unit::B => self.quotient as f64,
            Unit::KB => self.quotient as f64 + (self.remainder as f64 * SCALE_KB),
            Unit::MB => self.quotient as f64 + (self.remainder as f64 * SCALE_MB),
            Unit::GB => self.quotient as f64 + (self.remainder as f64 * SCALE_GB),
            Unit::TB => self.quotient as f64 + (self.remainder as f64 * SCALE_TB),
            Unit::PB => self.quotient as f64 + (self.remainder as f64 * SCALE_PB),
        }
    }

    pub fn quotient(&self) -> u64 {
        self.quotient
    }

    pub fn remainder(&self) -> u64 {
        self.remainder
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }
}

impl std::ops::Add<Storage> for Storage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let total_bytes = self.to_bytes() + other.to_bytes();
        Storage::from_bytes(total_bytes)
    }
}

impl std::ops::Add<&Storage> for Storage {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let total_bytes = self.to_bytes() + other.to_bytes();
        Storage::from_bytes(total_bytes)
    }
}

impl std::ops::Add<Storage> for &Storage {
    type Output = Storage;

    fn add(self, other: Storage) -> Self::Output {
        let total_bytes = self.to_bytes() + other.to_bytes();
        Storage::from_bytes(total_bytes)
    }
}

impl std::ops::Add<&Storage> for &Storage {
    type Output = Storage;

    fn add(self, other: &Storage) -> Self::Output {
        let total_bytes = self.to_bytes() + other.to_bytes();
        Storage::from_bytes(total_bytes)
    }
}

impl std::fmt::Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.to_float();
        write!(f, "{:.2} {}", value, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes() {
        assert_eq!(Storage::from_bytes(0), Storage::new(0, 0, Unit::B));

        assert_eq!(Storage::from_bytes(1), Storage::new(1, 0, Unit::B));
        assert_eq!(Storage::from_bytes(512), Storage::new(512, 0, Unit::B));
        assert_eq!(Storage::from_bytes(1023), Storage::new(1023, 0, Unit::B));

        assert_eq!(Storage::from_bytes(1024), Storage::new(1, 0, Unit::KB));
        assert_eq!(
            Storage::from_bytes(1024 * 1024),
            Storage::new(1, 0, Unit::MB)
        );
        assert_eq!(
            Storage::from_bytes(1024 * 1024 * 1024),
            Storage::new(1, 0, Unit::GB)
        );
        assert_eq!(
            Storage::from_bytes(1024 * 1024 * 1024 * 1024),
            Storage::new(1, 0, Unit::TB)
        );
        assert_eq!(
            Storage::from_bytes(1024 * 1024 * 1024 * 1024 * 1024),
            Storage::new(1, 0, Unit::PB)
        );

        assert_eq!(Storage::from_bytes(1536), Storage::new(1, 512, Unit::KB));
        assert_eq!(
            Storage::from_bytes(1024 * 1024 + 512),
            Storage::new(1, 512, Unit::MB)
        );
        assert_eq!(
            Storage::from_bytes(3 * 1024 * 1024 + 256),
            Storage::new(3, 256, Unit::MB)
        );
        assert_eq!(
            Storage::from_bytes(5 * 1024 * 1024 * 1024 + 1024 * 1024),
            Storage::new(5, 1024 * 1024, Unit::GB)
        );

        assert_eq!(
            Storage::from_bytes(1024 - 1),
            Storage::new(1023, 0, Unit::B)
        );
        assert_eq!(
            Storage::from_bytes(1024 * 1024 - 1),
            Storage::new(1023, 1023, Unit::KB)
        );

        assert_eq!(
            Storage::from_bytes(10 * 1024 * 1024 * 1024 * 1024 * 1024),
            Storage::new(10, 0, Unit::PB)
        );
        assert_eq!(
            Storage::from_bytes(100 * 1024 * 1024 * 1024 * 1024 * 1024 + 512),
            Storage::new(100, 512, Unit::PB)
        );
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(Storage::new(0, 0, Unit::B).to_bytes(), 0);
        assert_eq!(Storage::new(1, 0, Unit::B).to_bytes(), 1);
        assert_eq!(Storage::new(1023, 0, Unit::B).to_bytes(), 1023);

        assert_eq!(Storage::new(1, 0, Unit::KB).to_bytes(), 1024);
        assert_eq!(Storage::new(1, 0, Unit::MB).to_bytes(), 1024 * 1024);
        assert_eq!(Storage::new(1, 0, Unit::GB).to_bytes(), 1024 * 1024 * 1024);
        assert_eq!(
            Storage::new(1, 0, Unit::TB).to_bytes(),
            1024 * 1024 * 1024 * 1024
        );
        assert_eq!(
            Storage::new(1, 0, Unit::PB).to_bytes(),
            1024 * 1024 * 1024 * 1024 * 1024
        );

        assert_eq!(Storage::new(1, 512, Unit::KB).to_bytes(), 1536);
        assert_eq!(Storage::new(1, 512, Unit::MB).to_bytes(), 1024 * 1024 + 512);
        assert_eq!(
            Storage::new(3, 256, Unit::MB).to_bytes(),
            3 * 1024 * 1024 + 256
        );
        assert_eq!(
            Storage::new(5, 1024 * 1024, Unit::GB).to_bytes(),
            5 * 1024 * 1024 * 1024 + 1024 * 1024
        );
        assert_eq!(
            Storage::new(100, 512, Unit::PB).to_bytes(),
            100 * 1024 * 1024 * 1024 * 1024 * 1024 + 512
        );

        assert_eq!(
            Storage::new(1023, 1023, Unit::KB).to_bytes(),
            1023 * 1024 + 1023
        );
        assert_eq!(
            Storage::new(10, 0, Unit::TB).to_bytes(),
            10 * 1024 * 1024 * 1024 * 1024
        );
    }

    #[test]
    fn test_round_trip() {
        let test_cases = vec![
            0,
            1,
            512,
            1023,
            1024,
            1025,
            1536,
            1024 * 1024 - 1,
            1024 * 1024,
            1024 * 1024 + 512,
            3 * 1024 * 1024 + 256,
            1024 * 1024 * 1024 - 1,
            1024 * 1024 * 1024,
            5 * 1024 * 1024 * 1024 + 1024 * 1024,
            1024 * 1024 * 1024 * 1024,
            10 * 1024 * 1024 * 1024 * 1024 * 1024,
            100 * 1024 * 1024 * 1024 * 1024 * 1024 + 512,
        ];

        for bytes in test_cases {
            let unit = Storage::from_bytes(bytes);
            assert_eq!(
                unit.to_bytes(),
                bytes,
                "Round trip failed for {} bytes, got {:?}",
                bytes,
                unit
            );
        }
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(Storage::from_bytes(1023), Storage::new(1023, 0, Unit::B));
        assert_eq!(Storage::from_bytes(1024), Storage::new(1, 0, Unit::KB));
        assert_eq!(Storage::from_bytes(1025), Storage::new(1, 1, Unit::KB));

        let mb_boundary = 1024 * 1024;
        assert_eq!(
            Storage::from_bytes(mb_boundary - 1),
            Storage::new(1023, 1023, Unit::KB)
        );
        assert_eq!(
            Storage::from_bytes(mb_boundary),
            Storage::new(1, 0, Unit::MB)
        );
        assert_eq!(
            Storage::from_bytes(mb_boundary + 1),
            Storage::new(1, 1, Unit::MB)
        );

        let gb_boundary = 1024 * 1024 * 1024;
        assert_eq!(
            Storage::from_bytes(gb_boundary - 1),
            Storage::new(1023, 1023 * 1024 + 1023, Unit::MB)
        );
        assert_eq!(
            Storage::from_bytes(gb_boundary),
            Storage::new(1, 0, Unit::GB)
        );
        assert_eq!(
            Storage::from_bytes(gb_boundary + 1),
            Storage::new(1, 1, Unit::GB)
        );

        let tb_boundary = 1024_u64 * 1024 * 1024 * 1024;
        assert_eq!(
            Storage::from_bytes(tb_boundary),
            Storage::new(1, 0, Unit::TB)
        );

        let pb_boundary = 1024_u64 * 1024 * 1024 * 1024 * 1024;
        assert_eq!(
            Storage::from_bytes(pb_boundary),
            Storage::new(1, 0, Unit::PB)
        );
    }

    #[test]
    fn test_to_float() {
        const EPSILON: f64 = 1e-10;

        let value = Storage::new(0, 0, Unit::B).to_float();
        assert!((value - 0.0).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::B).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(512, 0, Unit::B).to_float();
        assert!((value - 512.0).abs() < EPSILON);

        let value = Storage::new(1023, 0, Unit::B).to_float();
        assert!((value - 1023.0).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::KB).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(5, 0, Unit::KB).to_float();
        assert!((value - 5.0).abs() < EPSILON);

        let value = Storage::new(1, 512, Unit::KB).to_float();
        assert!((value - 1.0 - (512.0 / 1024.0)).abs() < EPSILON);

        let value = Storage::new(2, 256, Unit::KB).to_float();
        assert!((value - 2.0 - (256.0 / 1024.0)).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::MB).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(3, 0, Unit::MB).to_float();
        assert!((value - 3.0).abs() < EPSILON);

        let value = Storage::new(1, 512, Unit::MB).to_float();
        let expected = 1.0 + (512.0 / (1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);

        let value = Storage::new(3, 256, Unit::MB).to_float();
        let expected = 3.0 + (256.0 / (1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::GB).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(5, 0, Unit::GB).to_float();
        assert!((value - 5.0).abs() < EPSILON);

        let value = Storage::new(5, 1024 * 1024, Unit::GB).to_float();
        let expected = 5.0 + ((1024.0 * 1024.0) / (1024.0 * 1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::TB).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(10, 0, Unit::TB).to_float();
        assert!((value - 10.0).abs() < EPSILON);

        let value = Storage::new(2, 512, Unit::TB).to_float();
        let expected = 2.0 + (512.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);

        let value = Storage::new(1, 0, Unit::PB).to_float();
        assert!((value - 1.0).abs() < EPSILON);

        let value = Storage::new(100, 0, Unit::PB).to_float();
        assert!((value - 100.0).abs() < EPSILON);

        let value = Storage::new(100, 512, Unit::PB).to_float();
        let expected = 100.0 + (512.0 / (1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);
    }

    #[test]
    fn test_to_float_round_trip() {
        const EPSILON: f64 = 1e-5;

        let test_cases = vec![
            Storage::new(0, 0, Unit::B),
            Storage::new(1, 0, Unit::B),
            Storage::new(512, 0, Unit::B),
            Storage::new(1023, 0, Unit::B),
            Storage::new(1, 0, Unit::KB),
            Storage::new(1, 512, Unit::KB),
            Storage::new(2, 256, Unit::KB),
            Storage::new(1, 0, Unit::MB),
            Storage::new(1, 512, Unit::MB),
            Storage::new(3, 256, Unit::MB),
            Storage::new(1, 0, Unit::GB),
            Storage::new(5, 1024 * 1024, Unit::GB),
            Storage::new(1, 0, Unit::TB),
            Storage::new(2, 512, Unit::TB),
            Storage::new(1, 0, Unit::PB),
            Storage::new(100, 512, Unit::PB),
        ];

        for storage in test_cases {
            let bytes = storage.to_bytes();
            let value = storage.to_float();

            let calculated_bytes = match storage.unit {
                Unit::B => value,
                Unit::KB => value * 1024.0,
                Unit::MB => value * 1024.0 * 1024.0,
                Unit::GB => value * 1024.0 * 1024.0 * 1024.0,
                Unit::TB => value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
                Unit::PB => value * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            };

            let diff = (calculated_bytes - bytes as f64).abs();
            assert!(
                diff < EPSILON * bytes as f64 || diff < 1.0,
                "Round trip failed for {:?}: expected {} bytes, got {} (diff: {})",
                storage,
                bytes,
                calculated_bytes,
                diff
            );
        }
    }

    #[test]
    fn test_to_float_edge_cases() {
        const EPSILON: f64 = 1e-10;

        let value = Storage::new(1023, 1023, Unit::KB).to_float();
        let expected = 1023.0 + (1023.0 / 1024.0);
        assert!((value - expected).abs() < EPSILON);

        let value = Storage::new(1000, 0, Unit::PB).to_float();
        assert!((value - 1000.0).abs() < EPSILON);

        let value = Storage::new(10, 1024 * 1024 * 1023, Unit::GB).to_float();
        let expected = 10.0 + ((1024.0 * 1024.0 * 1023.0) / (1024.0 * 1024.0 * 1024.0));
        assert!((value - expected).abs() < EPSILON);
    }
}
