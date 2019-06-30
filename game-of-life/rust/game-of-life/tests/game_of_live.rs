#[cfg(test)]
mod tests {
    use std::fs;

    use game_of_life::*;
    use game_of_life::CellType::Dead;
    use game_of_life::CellType::Alive;

    #[test]
    fn a_block_evolves_into_a_block() {
        let seed = load_grid("block");

        let new_generation = evolve(&seed);

        assert_eq!(new_generation, load_grid("block"));
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

    #[test]
    fn a_horizontal_oscillator_evolves_into_a_vertical_one() {
        let horizontal_oscillator = vec![
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Alive, Alive, Alive, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
        ];

        let new_generation = evolve(&horizontal_oscillator);

        let vertical_oscillator = vec![
            vec![Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead],
        ];
        assert_eq!(new_generation, vertical_oscillator);
    }

    #[test]
    fn a_packed_toad_evolves_into_an_open_one() {
        let pack_toad = vec![
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Alive, Alive, Alive, Dead],
            vec![Dead, Alive, Alive, Alive, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
        ];

        let new_generation = evolve(&pack_toad);

        let open_toad = vec![
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Alive, Dead, Dead, Alive, Dead],
            vec![Dead, Alive, Dead, Dead, Alive, Dead],
            vec![Dead, Dead, Alive, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
        ];
        assert_eq!(new_generation, open_toad);
    }

    #[test]
    fn an_open_toad_evolves_into_a_packed_one() {
        let open_toad = vec![
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Alive, Dead, Dead],
            vec![Dead, Alive, Dead, Dead, Alive, Dead],
            vec![Dead, Alive, Dead, Dead, Alive, Dead],
            vec![Dead, Dead, Alive, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
        ];

        let new_generation = evolve(&open_toad);

        let packed_toad = vec![
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Alive, Alive, Alive, Dead],
            vec![Dead, Alive, Alive, Alive, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
            vec![Dead, Dead, Dead, Dead, Dead, Dead],
        ];
        assert_eq!(new_generation, packed_toad);
    }

    fn load_grid(filename: &str) -> Grid {
        fs::read_to_string(
                format!("../../gol-patterns/{}.txt", filename)
            )
            .expect("Could not read file")
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|&line| line
                 .chars()
                 .map(|c| if c == 'o' { Alive } else { Dead  })
                 .collect::<Vec<CellType>>()
            )
            .collect::<Vec<Vec<CellType>>>()
    }
}
