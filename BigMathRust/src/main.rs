use core::panic;

struct BigNumber {
    value: Vec<u8>, 
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

fn main() {

    // set_hex + get_hex tests

    // let test_cases = [
    //     "1A2B3C4D5E6F",
    //     "0",
    //     "123456789ABCDE0F",
    //     "123456789ABCDEF",
    //     "1",
    //     "F",
    //     "1000",
    //     "1000000000000000000000000001",
    //     "327AFBC47385647865983446589346578238CFFFFAAAAAAAA",
    // ];

    // for hex_str in &test_cases {
    //     let mut big_num = BigNumber::new();
    //     big_num.set_hex(hex_str);
    //     let result = big_num.get_hex();

    //     let hex_str_as_string = hex_str.to_string();

    //     println!("Input: {:>30} => Output: {}", hex_str, result);
    //     println!("{:?}", big_num.value); 

    //     assert_eq!(hex_str_as_string, result);
    // }

    //inv test

    // let mut big_num = BigNumber::new();
    // big_num.set_hex("1A2B3C4D5E6F");
    // big_num.inv();
    // println!("{}", big_num.get_hex());

    //xor test

    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    // number_b.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");

    // number_a.xor(&number_b);

    // let expected_result = "1182D8299C0EC40CA8BF3F49362E95E4ECEDAF82BFD167988972412095B13DB8";
    
    // println!("{}", number_a.get_hex());
    // assert_eq!(number_a.get_hex(), expected_result);


    //or test

    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    // number_b.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");

    // number_a.or(&number_b);

    // let expected_result = "51BFF8AD9CAFD72EABFFBFC9BEFFFFFFFCFFBFFAFFDD779AFDF3D7FDF7F73FBC";
    
    // println!("{}", number_a.get_hex());
    // assert_eq!(number_a.get_hex(), expected_result);

    //and test

    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    // number_b.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");

    // number_a.and(&number_b);

    // let expected_result = "403D208400A113220340808088D16A1B10121078400C1002748196DD62460204";
    
    // println!("{}", number_a.get_hex());
    // assert_eq!(number_a.get_hex(), expected_result);

    //shirt_r test

    // let mut big_num = BigNumber::new();
    // big_num.set_hex("1A2B3C4D5E6F");
    // big_num.shift_r(1);
    // println!("{}", big_num.get_hex());

    //shirt_l test

    // let mut big_num = BigNumber::new();
    // big_num.set_hex("1A2B3C4D5E6F");
    // big_num.shift_l(1);
    // println!("{}", big_num.get_hex());

    //add test

    
    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("36F028580BB02CC8272A9A020F4200E346E276AE664E45EE80745574E2F5AB80");
    // number_b.set_hex("70983D692F648185FEBE6D6FA607630AE68649F7E6FC45B94680096C06E4FADB");

    // number_a.add(&number_b);

    // let expected_result = "A78865C13B14AE4E25E90771B54963EE2D68C0A64D4A8BA7C6F45EE0E9DAA65B";
    
    // println!("{}", number_a.get_hex());
    // assert_eq!(number_a.get_hex(), expected_result);

    //substarct test

    
    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("33CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    // number_b.set_hex("22E962951CB6CD2CE279AB0E2095825C141D48EF3CA9DABF253E38760B57FE03");

    // number_a.subtract(&number_b);

    // let expected_result = "10E570324E6FFDBC6B9C813DEC968D9BAD134BC0DBB061530934F4E59C2700B9";
    
    // println!("{}", number_a.get_hex());
    // assert_eq!(number_a.get_hex(), expected_result);

    
    //compare test

    
    // let mut number_a = BigNumber::new();
    // let mut number_b = BigNumber::new();
    
    // number_a.set_hex("10E570324E6FFDBC6B9C813DEC968D9BAD134BC0DBB061530934F4E59C2700B9");
    // number_b.set_hex("22E962951CB6CD2CE279AB0E2095825C141D48EF3CA9DABF253E38760B57FE03");

    
    // println!("{}", number_a.compare(&number_b));
   

    //modulo test

    
    let mut number_a = BigNumber::new();
    let mut number_b = BigNumber::new();
    
    number_a.set_hex("75CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    number_b.set_hex("22E962951CB6CD2CE279AB0E2095825C141D48EF3CA9DABF253E38760B57FE03");

    number_a.modulo(&number_b);

    let expected_result = "18B0F8F44C02C72875557DBEC754F85EF80BBB57B9B7B09ABA04F8BF360F129E";
    
    println!("{}", number_a.get_hex());
    assert_eq!(number_a.get_hex(), expected_result);


}
