use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Grid { rows: 4, cols: 4 }
    }
}

#[derive(Default)]
pub struct Board {
    pub grid: Grid,
}

#[derive(Debug, Copy, Clone)]
pub struct Value(u32);

impl Value {
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        assert!([2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048].contains(&value));
        Value(value)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.0, rhs.0);
        Value::from(self.0 + rhs.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub value: Value,
}
