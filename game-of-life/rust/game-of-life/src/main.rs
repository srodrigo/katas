fn main() {
    println!("Hello, world!");
}
#[derive(Debug, PartialEq, Clone)]
enum CellType {
    Dead,
    Alive
}

type Grid = Vec<Vec<CellType>>;

fn evolve(seed: &Grid) -> Grid {
    return seed.to_vec();
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

    #[test]
    fn a_vertical_oscillator_evolves_into_a_horizonal_one() {
        let seed = vec![
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
        ];

        let new_generation = evolve(&seed);

        let horizontal_oscillator = vec![
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Alive, Alive, Alive, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
        ];
        assert_eq!(new_generation, horizontal_oscillator);
    }
}
