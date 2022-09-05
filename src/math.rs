
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
    let mut v: Vec<String> = Vec::new();
    if sol.len()%4 != 0 {
        for _i in 0..(4 - sol.len()%4) { //meth
            sol.insert(0, '0');
        }
    }
    for i in 0..sol.len()/4 {
        v.insert(i, sol.get(i*4..(i+1)*4).expect("Oopsie").to_string());
    }
    sol = "".to_string();
    for i in &v {
        let mut val = 0;
        let mut t = i.chars().rev();
        for i in 0..4 {
            if t.next() == Some('1') {
                val += 2_i32.pow(i);
            } else {
                continue;
            }
        }
        if val < 10 {
            sol += val.to_string().as_str();
        }
        else {
            sol += match val {
                10 => "A",
                11 => "B",
                12 => "C",
                13 => "D",
                14 => "E",
                15 => "F",
                _ => "",
            };
        }
    }
    sol
}

#[test]
fn test_bin() {
    assert_eq!(to_bin(5), "101");
    assert_eq!(to_bin(6), "110");
    assert_eq!(to_bin(13), "1101");
    assert_eq!(to_bin(256), "100000000");
}