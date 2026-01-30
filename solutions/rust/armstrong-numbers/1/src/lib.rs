pub fn is_armstrong_number(num: u32) -> bool {
    let n = num.to_string();
    let len = n.len() as u32;
    let mut sum_pow= 0;
    for c in n.chars() {
        let digit = c.to_digit(10).unwrap();
        sum_pow += digit.pow(len);
        }
    sum_pow == num
    }
