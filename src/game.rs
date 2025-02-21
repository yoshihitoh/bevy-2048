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
