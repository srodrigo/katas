#[cfg(test)]
mod tests {
    use std::fs;

    use game_of_life::*;
    use game_of_life::CellType::Dead;
    use game_of_life::CellType::Alive;

    #[test]
    fn a_block_evolves_into_a_block() {
        assert_eq!(
            evolve(&load_grid("block")),
            load_grid("block")
        );
    }

    #[test]
    fn a_vertical_oscillator_evolves_into_a_horizonal_one() {
        assert_eq!(
            evolve(&load_grid("vertical-oscillator")),
            load_grid("horizontal-oscillator")
        );
    }

    #[test]
    fn a_horizontal_oscillator_evolves_into_a_vertical_one() {
        assert_eq!(
            evolve(&load_grid("horizontal-oscillator")),
            load_grid("vertical-oscillator")
        );
    }

    #[test]
    fn a_packed_toad_evolves_into_an_open_one() {
        assert_eq!(
            evolve(&load_grid("packed-toad")),
            load_grid("open-toad")
        );
    }

    #[test]
    fn an_open_toad_evolves_into_a_packed_one() {
        assert_eq!(
            evolve(&load_grid("open-toad")),
            load_grid("packed-toad")
        );
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
