use std::collections::HashMap;

fn hex_decode(s: String) -> String {
    s.chars().map(|c| {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _   => ""
        }
    }).collect()
}
fn main() {
    let tmp = "38006F45291200";
    let input = hex_decode(tmp.to_string());
    let mut mainn = 0;
    let mut mainc = 1;
    while mainc > 0 {
        mainc -= 1;
        let version = isize::from_str_radix(&input[mainn..mainn+3], 2).unwrap();
        let typeid = isize::from_str_radix(&input[mainn+3..mainn+6], 2).unwrap();
        mainn += 6;

        if typeid == 4 {
            let mut number = "".to_string();
            let mut n = 6;
            loop {
                number += &input[n+1..n+5];
                mainn += 5;
                if input.chars().nth(n).unwrap() == '0' {
                    break;
                }
                n += 5;
            }
            println!("{}", isize::from_str_radix(&number, 2).unwrap());
        } else {
             if input.chars().nth(mainn+6).unwrap() == '0' {
                // isize::from_str_radix(&input[mainn+7..mainn+22], 2).unwrap();
             } else {
                mainc += isize::from_str_radix(&input[mainn+7..mainn+18], 2).unwrap();
             };
        }
        break;
    }
}
