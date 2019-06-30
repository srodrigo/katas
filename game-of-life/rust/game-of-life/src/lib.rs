#[derive(Debug, PartialEq, Clone)]
pub enum CellType {
    Dead,
    Alive
}

pub type Grid = Vec<Vec<CellType>>;
type CellPos = (usize, usize);

pub fn evolve(seed: &Grid) -> Grid {
    let mut new_generation = seed.to_vec();

    for (y, row) in seed.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let position = (x, y);

            if is_alive_at(seed, position) && is_underpopulated_at(seed, position) {
                kill_cell_at(&mut new_generation, position);
            }
            if is_alive_at(seed, position) && is_overpopulated_at(seed, position) {
                kill_cell_at(&mut new_generation, position);
            }
            if is_dead_at(seed, position) && is_reproducible_at(seed, position) {
                revive_cell_at(&mut new_generation, position);
            }
        }
    }

    new_generation
}

fn count_live_neighbours(grid: &Grid, position: CellPos) -> u16 {
    let mut num_neighbours = 0;

    let cell_y = position.1;
    let cell_x = position.0;

    let y_start = if cell_y > 0 { cell_y - 1 } else { 0 };
    let y_end = if cell_y < grid.len() - 1 { cell_y + 2 } else { grid.len() };
    let x_start = if cell_x > 0 { cell_x - 1 } else { 0 };
    let x_end = if cell_x < grid[0].len() - 1 { cell_x + 2 } else { grid[0].len() };

    for y in y_start..y_end {
        for x in x_start..x_end {
            if !(x == cell_x && y == cell_y) && *at(grid, (x, y)) == CellType::Alive {
                num_neighbours = num_neighbours + 1;
            }
        }
    }

    num_neighbours
}

fn is_alive_at(grid: & Grid, position: CellPos) -> bool {
    *at(grid, position) == CellType::Alive
}

fn is_dead_at(grid: & Grid, position: CellPos) -> bool {
    *at(grid, position) == CellType::Dead
}

fn kill_cell_at(grid: &mut Grid, position: CellPos) {
    set(grid, position, CellType::Dead);
}

fn revive_cell_at(grid: &mut Grid, position: CellPos) {
    set(grid, position, CellType::Alive);
}

fn is_underpopulated_at(grid: &Grid, position: CellPos) -> bool {
    count_live_neighbours(grid, position) < 2
}

fn is_overpopulated_at(grid: &Grid, position: CellPos) -> bool {
    count_live_neighbours(grid, position) > 3
}

fn is_reproducible_at(grid: &Grid, position: CellPos) -> bool {
    count_live_neighbours(grid, position) == 3
}

fn at(grid: &Grid, position: CellPos) -> &CellType {
    &grid[position.1][position.0]
}

fn set(grid: & mut Grid, position: CellPos, cell_type: CellType) {
    grid[position.1][position.0] = cell_type;
}