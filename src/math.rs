
//map a value from one range to its analogue in another
pub fn map(val: f32, from_beg: f32, to_beg: f32, from_end: f32, to_end: f32) -> f32 {
    val*(to_end - from_end)/(to_beg - from_beg)
}

fn to_bin(mut val: u32) -> String {
    let mut sol = String::new();

    while val != 0 {
        sol.push(char::from_digit(val%2, 10)
            .expect("Fak"));
        val = val/2;
    }
    sol.chars().rev().collect::<String>()
}

pub fn to_hex(num: u32) -> String {
    let mut sol = to_bin(num);
    for i in 0..sol.len() {
        todo!()
    }
    sol
}

#[test]
fn test_bin() {
    assert_eq!(to_bin(5), "101");
    assert_eq!(to_bin(6), "110");
    assert_eq!(to_bin(13), "1101");
}