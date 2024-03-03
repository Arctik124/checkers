use std::error::Error;

pub mod models;

// pub fn run(game: Game) -> Result<(), Box<dyn Error>> {}

fn parse_input(input: &str) -> Result<((u8, u8), (u8, u8)), &'static str> {
    let mut result = ((0, 0), (0, 0));
    let splitted: Vec<char> = input.to_lowercase().chars().collect();
    if splitted.len() > 5 {
        return Err("input len is greater than allowed");
    }
    result.0 .0 = char_to_index(splitted[0])?;
    result.0 .1 = char_to_index(splitted[1])?;
    result.1 .0 = char_to_index(splitted[3])?;
    result.1 .1 = char_to_index(splitted[4])?;
    Ok(result)
}

fn char_to_index(c: char) -> Result<u8, &'static str> {
    match c {
        'a'..='h' => Ok((c as u8) - ('a' as u8)),
        '1'..='8' => Ok((c.to_digit(10).unwrap() - 1) as u8),
        _ => Err("cannot convert char {c} to index"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let input = "a1 a3";
        assert_eq!(((0, 0), (0, 2)), parse_input(input).unwrap())
    }

    #[test]
    fn char_to_index_test() {
        let c = 'a';
        assert_eq!(0, char_to_index(c).unwrap())
    }
}
