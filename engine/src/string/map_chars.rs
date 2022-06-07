use super::Cow;

pub type MapChars = &'static [(char, char)];

pub(super) const MAP_CHAR_FORP: MapChars = &[
    ('Ќ', '♣'),
    ('Ћ', '♦'),
    ('Џ', '♥'),
    ('ђ', '♠'),
];

fn select((a, b): &(char, char), first: bool) -> char {
    if first {
        *a
    } else {
        *b
    }
}

fn from(tuple: &(char, char), left_to_right: bool) -> char {
    select(tuple, left_to_right)
}

fn to(tuple: &(char, char), left_to_right: bool) -> char {
    select(tuple, !left_to_right)
}


pub(super) fn map_chars<'a>(string: Cow<'a, str>, mapper: &[(char, char)], left_to_right: bool) -> Cow<'a, str> {
    let find = |char| mapper.iter().find(|tuple| from(tuple, left_to_right) == char);
    let first_to_map = string.chars().position(|char| find(char).is_some());
    if let Some(first) = first_to_map {
        let mut new_string = string[..first].to_owned();
        new_string.extend(string[first..].chars().map(|char| {
            find(char).map(|tuple| to(tuple, left_to_right)).unwrap_or(char)
        }));
        new_string.into()
    } else {
        string
    }
}
