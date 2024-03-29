use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub enum CellType {
    Dead,
    Alive,
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

    for y in neighbours_range(cell_y, num_rows(grid)) {
        for x in neighbours_range(cell_x, num_columns(grid)) {
            let position_checked = (x, y);
            if not_equal(position_checked, position) && is_alive_at(grid, position_checked) {
                num_neighbours = num_neighbours + 1;
            }
        }
    }

    num_neighbours
}

fn num_columns(grid: &Grid) -> usize {
    grid[0].len()
}

fn num_rows(grid: &Grid) -> usize {
    grid.len()
}

fn not_equal(position_a: CellPos, position_b: CellPos) -> bool {
    !(position_a.0 == position_b.0 && position_a.1 == position_b.1)
}

fn neighbours_range(position: usize, limit: usize) -> Range<usize> {
    Range {
        start: if position > 0 { position - 1 } else { 0 },
        end: if position < limit - 1 {
            position + 2
        } else {
            limit
        },
    }
}

fn is_alive_at(grid: &Grid, position: CellPos) -> bool {
    at(grid, position) == CellType::Alive
}

fn is_dead_at(grid: &Grid, position: CellPos) -> bool {
    at(grid, position) == CellType::Dead
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

fn at(grid: &Grid, position: CellPos) -> CellType {
    grid[position.1][position.0].clone()
}

fn set(grid: &mut Grid, position: CellPos, cell_type: CellType) {
    grid[position.1][position.0] = cell_type;
}
