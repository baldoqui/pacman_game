use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum CellType {
    Empty,
    Wall,
    Dot,
    PowerUp,
}

#[derive(Component, Clone, Copy)]
pub struct GridCell {
    pub cell_type: CellType,
    pub x: i32,
    pub y: i32,
}

impl GridCell {
    pub fn new(cell_type: CellType, x: i32, y: i32) -> Self {
        GridCell { cell_type, x, y }
    }
}

#[derive(Component)]
pub struct GameMap {
    pub grid: Vec<Vec<GridCell>>,
    pub width: usize,
    pub height: usize,
}

impl GameMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(height);
        
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let cell_type = if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    CellType::Wall
                } else {
                    CellType::Empty
                };
                
                row.push(GridCell::new(cell_type, x as i32, y as i32));
            }
            grid.push(row);
        }

        GameMap {
            grid,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&GridCell> {
        if x < self.width && y < self.height {
            Some(&self.grid[y][x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut GridCell> {
        if x < self.width && y < self.height {
            Some(&mut self.grid[y][x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: GridCell) -> bool {
        if x < self.width && y < self.height {
            self.grid[y][x] = cell;
            true
        } else {
            false
        }
    }

    pub fn classic() -> Self {
        let mut map = GameMap::new(16, 16);

        for i in 1..map.width - 1 {
            for j in 1..map.height - 1 {
                if let Some(cell) = map.get(i, j) {
                    if matches!(cell.cell_type, CellType::Empty) {
                        map.set(i, j, GridCell::new(CellType::Dot, i as i32, j as i32));
                    }
                }
            }
        }

        map.set(1, 1, GridCell::new(CellType::PowerUp, 1, 1));
        map.set(map.width - 2, 1, GridCell::new(CellType::PowerUp, (map.width - 2) as i32, 1));
        map.set(1, map.height - 2, GridCell::new(CellType::PowerUp, 1, (map.height - 2) as i32));
        map.set(map.width - 2, map.height - 2, GridCell::new(CellType::PowerUp, (map.width - 2) as i32, (map.height - 2) as i32));

        map
    }
}