fn main() {
    println!("Hello, world!");
}
#[derive(Debug, PartialEq)]
enum CellType {
    Dead,
    Alive
}

type Grid = Vec<Vec<CellType>>;

fn evolve(seed: &Grid) -> Grid {
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::CellType::*;

    #[test]
    fn a_block_evolves_into_a_block() {
        let seed = vec![
            vec![Dead, Dead, Dead, Dead],
            vec![Dead, Alive, Alive, Dead],
            vec![Dead, Alive, Alive, Dead],
            vec![Dead, Dead, Dead, Dead],
        ];

        let new_generation = evolve(&seed);

        assert_eq!(new_generation, seed);
    }
}
