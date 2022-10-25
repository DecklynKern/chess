pub const A8: usize = 26;
pub const B8: usize = 27;
pub const C8: usize = 28;
pub const D8: usize = 29;
pub const E8: usize = 30;
pub const F8: usize = 31;
pub const G8: usize = 32;
pub const H8: usize = 33;

pub const A7: usize = 38;
pub const B7: usize = 39;
pub const C7: usize = 40;
pub const D7: usize = 41;
pub const E7: usize = 42;
pub const F7: usize = 43;
pub const G7: usize = 44;
pub const H7: usize = 45;

pub const A6: usize = 50;
pub const B6: usize = 51;
pub const C6: usize = 52;
pub const D6: usize = 53;
pub const E6: usize = 54;
pub const F6: usize = 55;
pub const G6: usize = 56;
pub const H6: usize = 57;

pub const A5: usize = 62;
pub const B5: usize = 63;
pub const C5: usize = 64;
pub const D5: usize = 65;
pub const E5: usize = 66;
pub const F5: usize = 67;
pub const G5: usize = 68;
pub const H5: usize = 69;

pub const A4: usize = 74;
pub const B4: usize = 75;
pub const C4: usize = 76;
pub const D4: usize = 77;
pub const E4: usize = 78;
pub const F4: usize = 79;
pub const G4: usize = 80;
pub const H4: usize = 81;

pub const A3: usize = 86;
pub const B3: usize = 87;
pub const C3: usize = 88;
pub const D3: usize = 89;
pub const E3: usize = 90;
pub const F3: usize = 91;
pub const G3: usize = 92;
pub const H3: usize = 93;

pub const A2: usize = 98;
pub const B2: usize = 99;
pub const C2: usize = 100;
pub const D2: usize = 101;
pub const E2: usize = 102;
pub const F2: usize = 103;
pub const G2: usize = 104;
pub const H2: usize = 105;

pub const A1: usize = 110;
pub const B1: usize = 111;
pub const C1: usize = 112;
pub const D1: usize = 113;
pub const E1: usize = 114;
pub const F1: usize = 115;
pub const G1: usize = 116;
pub const H1: usize = 117;

pub const VALID_SQUARES: [usize; 64] = [
    A8, B8, C8, D8, E8, F8, G8, H8, 
    A7, B7, C7, D7, E7, F7, G7, H7, 
    A6, B6, C6, D6, E6, F6, G6, H6, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A1, B1, C1, D1, E1, F1, G1, H1, 
];

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
    }, 10 - idx / 12)

}

pub fn index_to_an(idx: usize) -> String {

    let rank = 10 - idx / 12;
    let file = String::from("abcdefgh").chars().nth(idx % 12 - 2).unwrap();

    return format!("{}{}", file, rank);

}