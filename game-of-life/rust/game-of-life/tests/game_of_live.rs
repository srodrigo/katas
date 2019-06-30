#[cfg(test)]
mod tests {
    use std::fs;

    use game_of_life::*;
    use game_of_life::CellType::Dead;
    use game_of_life::CellType::Alive;

    macro_rules! game_of_life_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(evolve(&input), expected);
                }
            )*
        }
    }

    game_of_life_tests! {
        a_block_evolves_into_a_block: (block(), block()),
        a_vertical_oscillator_evolves_into_a_horizonal_one: (vertical_oscillator(), horizontal_oscillator()),
        a_horizontal_oscillator_evolves_into_a_vertical_one: (horizontal_oscillator(), vertical_oscillator()),
        a_packed_toad_evolves_into_an_open_one: (packed_toad(), open_toad()),
        an_open_toad_evolves_into_a_packed_one: (open_toad(), packed_toad()),
    }

    fn block() -> Grid {
        load_grid("block")
    }

    fn vertical_oscillator() -> Grid {
        load_grid("vertical-oscillator")
    }

    fn horizontal_oscillator() -> Grid {
        load_grid("horizontal-oscillator")
    }

    fn packed_toad() -> Grid {
        load_grid("packed-toad")
    }

    fn open_toad() -> Grid {
        load_grid("open-toad")
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
