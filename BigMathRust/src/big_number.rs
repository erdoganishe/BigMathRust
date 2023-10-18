use core::panic;

pub struct BigNumber {
    pub value: Vec<u8>, 
}

impl BigNumber {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn set_hex(&mut self, hex_str: &str) {
        self.value.clear();
    
        let mut hex_chars: Vec<char> = hex_str.chars().collect();
        if hex_chars.len() % 2 != 0 {
            hex_chars.insert(0, '0');
        }
    
        let mut current_value: u128 = 0;
        let mut current_digit = 0;
        let mut digit_count = 0;
    
        for &hex_char in &hex_chars {
            current_digit = match hex_char {
                '0'..='9' => hex_char as u128 - b'0' as u128,
                'A'..='F' => hex_char as u128 - b'A' as u128 + 10,
                'a'..='f' => hex_char as u128 - b'a' as u128 + 10,
                _ => {
                    panic!("Invalid hex character: {}", hex_char);
                }
            };
    
            current_value = (current_value << 4) | current_digit;
            digit_count += 1;
    
            if digit_count == 2 {
                self.value.push((current_value & 0xFF) as u8);
                current_value >>= 8;
                digit_count = 0;
            }
        }
    
        if digit_count > 0 {
            self.value.push(current_value as u8);
        }
    }
    
    
    pub fn get_hex(&self) -> String {
        if self.value.is_empty() {
            return "00".to_string();
        }
    
        let mut hex_string = String::new();
    
        for (i, &byte) in self.value.iter().enumerate() {
            let byte_string = format!("{:02X}", byte);
            hex_string.push_str(&byte_string);
        }
    
        if let Some(first_char) = hex_string.chars().next() {
            if first_char == '0' {
                hex_string.remove(0);
            }
        }
    
        hex_string
    }

    pub fn compare(&self, other: &BigNumber) -> i32 {
        if self.get_hex() > other.get_hex() {
            return 1;
        } else if self.get_hex() < other.get_hex() {
            return -1;
        }

        0
    }
    
    
    
    
    pub fn inv(&mut self) {
        for byte in &mut self.value {
            *byte = !*byte;
        }
    }
    
    pub fn xor(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte ^= *other_byte;
        }
    }

    pub fn or(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte |= *other_byte;
        }
    }

    pub fn and(&mut self, other: &BigNumber) {
        for (byte, other_byte) in self.value.iter_mut().zip(other.value.iter()) {
            *byte &= *other_byte;
        }
    }

    pub fn shift_r(&mut self, n: usize) {
        let mut carry = 0;
        for byte in self.value.iter_mut() {
            let new_byte = (*byte >> n) | (carry << (8 - n));
            carry = *byte & ((1 << n) - 1);
            *byte = new_byte;
        }
    }

    pub fn shift_l(&mut self, n: usize) {
        let mut carry = 0;
        for byte in self.value.iter_mut().rev() {
            let new_byte = (*byte << n) | carry;
            carry = *byte >> (8 - n);
            *byte = new_byte;
        }
    }

    pub fn add(&mut self, other: &BigNumber) {
        let hex1 = self.get_hex();
        let hex2 = other.get_hex();
        let mut res: String = "".to_string();
        let mut carry: bool = false;
    
        for i in 0..hex1.len().max(hex2.len()) + 1 {
            let cur_char1 = hex1.chars().rev().nth(i).unwrap_or('0');
            let cur_char2 = hex2.chars().rev().nth(i).unwrap_or('0');
            let hex_values = "0123456789ABCDEF";
            let val1 = match hex_values.find(cur_char1) {
                Some(position) => position as u8,
                None => panic!(""),
            };
            let val2 = match hex_values.find(cur_char2) {
                Some(position) => position as u8,
                None => panic!(""),
            };
            
            let mut sum: u8 = val1 + val2;
    
            if carry {
                sum += 1;
                carry = false;
            }
    
            if sum > 15 {
                carry = true;
                sum -= 16;
            }
    
            let hex_char = match sum {
                0..=9 => (sum + b'0') as char,
                10..=15 => (sum - 10 + b'A') as char,
                _ => panic!(""),
            };
    
            res = hex_char.to_string() + &res;
                   }
        if let Some(first_char) = res.chars().next() {
            if first_char == '0' {
                res.remove(0);
            }
        }
        self.set_hex(&res);

    }
    
    pub fn subtract(&mut self, other: &BigNumber) {

        if (self.compare(other) == -1){
            panic!("");
        }
        let hex1 = self.get_hex();
        let hex2 = other.get_hex();
        let mut res: String = "".to_string();
        let mut borrow: bool = false;
        
        for i in 0..hex1.len().max(hex2.len()) {
            let cur_char1 = hex1.chars().rev().nth(i).unwrap_or('0');
            let cur_char2 = hex2.chars().rev().nth(i).unwrap_or('0');
            let hex_values = "0123456789ABCDEF";
            let val1 = match hex_values.find(cur_char1) {
                Some(position) => position as i32,
                None => panic!(""),
            };
            let val2 = match hex_values.find(cur_char2) {
                Some(position) => position as i32,
                None => panic!(""),
            };
    
            let mut difference = val1 - val2;
    
            if borrow {
                difference -= 1;
                borrow = false;
            }
    
            if difference < 0 {
                difference += 16;
                borrow = true;
            }
    
            let hex_char = match difference {
                0..=9 => (difference + b'0' as i32) as u8 as char,
                10..=15 => (difference - 10 + b'A' as i32) as u8 as char,
                _ => panic!(""),
            };
    
            res = hex_char.to_string() + &res;
        }
        if let Some(first_char) = res.chars().next() {
            if first_char == '0' {
                res.remove(0);
            }
        }
        self.set_hex(&res);
    }



    pub fn modulo(&mut self, other: &BigNumber) {
        if (other.get_hex() == "0") {
            panic!("Division by zero is undefined.");
        }    
        while self.compare(other)!=-1 {
            self.subtract(other);
        }      

    }

}

