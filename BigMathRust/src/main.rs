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
    
}

fn main() {

    // set_hex + get_hex tests

    // let test_cases = [
    //     "1A2B3C4D5E6F",
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


}
