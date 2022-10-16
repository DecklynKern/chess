pub fn long_an_to_index(long_an: String) -> usize {

    let mut chars = long_an.chars();
    (match chars.next().unwrap() {
        'a' => 2,
        'b' => 3,
        'c' => 4,
        'd' => 5,
        'e' => 6,
        'f' => 7,
        'g' => 8,
        'h' => 9,
        _ => unreachable!()
    } + 12 * (10 - chars.next().unwrap().to_digit(10).unwrap() as usize))
    
}

pub fn index_to_long_an(idx: usize) -> String {
    format!("{}{}", match idx % 12 {
        2 => "a",
        3 => "b",
        4 => "c",
        5 => "d",
        6 => "e",
        7 => "f",
        8 => "g",
        9 => "h",
        _ => unreachable!()
    }, (10 - idx / 12).to_string())

}

pub fn index_to_an(idx: usize) -> String {

    let rank = 10 - idx / 12;
    let file = String::from("abcdefgh").chars().nth(idx % 12 - 2).unwrap();

    return format!("{}{}", file, rank);

}