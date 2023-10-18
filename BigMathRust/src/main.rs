mod big_number;
use big_number::BigNumber;

fn main() {

    // set_hex + get_hex tests
    println!("set_hex + get_hex test");
    let test_cases = [
        "1A2B3C4D5E6F",
        "0",
        "123456789ABCDE0F",
        "123456789ABCDEF",
        "1",
        "F",
        "1000",
        "1000000000000000000000000001",
        "327AFBC47385647865983446589346578238CFFFFAAAAAAAA",
    ];

    for hex_str in &test_cases {
        let mut big_num = BigNumber::new();
        big_num.set_hex(hex_str);
        let result = big_num.get_hex();

        let hex_str_as_string = hex_str.to_string();

        println!("Input: {:>30} => Output: {}", hex_str, result);
        println!("{:?}", big_num.value); 

        assert_eq!(hex_str_as_string, result);
    }

    //inv test
    println!("inv test");
    let mut big_num_inv = BigNumber::new();
    big_num_inv.set_hex("1A2B3C4D5E6F");
    println!("{} INV ->", big_num_inv.get_hex());
    big_num_inv.inv();
    println!("{}", big_num_inv.get_hex());

    //xor test
    println!("xor test");
    let mut number_a_xor = BigNumber::new();
    let mut number_b_xor = BigNumber::new();
    
    number_a_xor.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    number_b_xor.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");
    println!("{} XOR {} ->", number_a_xor.get_hex(),number_b_xor.get_hex() );
    number_a_xor.xor(&number_b_xor);

    let expected_result_xor = "1182D8299C0EC40CA8BF3F49362E95E4ECEDAF82BFD167988972412095B13DB8";
    
    println!("{}", number_a_xor.get_hex());
    assert_eq!(number_a_xor.get_hex(), expected_result_xor);


    //or test
    println!("or test");
    let mut number_a_or = BigNumber::new();
    let mut number_b_or = BigNumber::new();
    
    number_a_or.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    number_b_or.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");
    println!("{} OR {} ->", number_a_or.get_hex(),number_b_or.get_hex() );
    number_a_or.or(&number_b_or);

    let expected_result_or = "51BFF8AD9CAFD72EABFFBFC9BEFFFFFFFCFFBFFAFFDD779AFDF3D7FDF7F73FBC";
    
    println!("{}", number_a_or.get_hex());
    assert_eq!(number_a_or.get_hex(), expected_result_or);

    //and test
    println!("and test");
    let mut number_a_and = BigNumber::new();
    let mut number_b_and = BigNumber::new();
    
    number_a_and.set_hex("51BF608414AD5726A3C1BEC098F77B1B54FFB2787F8D528A74C1D7FDE6470EA4");
    number_b_and.set_hex("403DB8AD88A3932A0B7E8189AED9EEFFB8121DFAC05C3512FDB396DD73F6331C");
    println!("{} AND {} ->", number_a_and.get_hex(),number_b_and.get_hex() );
    number_a_and.and(&number_b_and);

    let expected_result_and = "403D208400A113220340808088D16A1B10121078400C1002748196DD62460204";
    
    println!("{}", number_a_and.get_hex());
    assert_eq!(number_a_and.get_hex(), expected_result_and);

    //shirt_r test
    println!("shirt_r test");
    let mut big_num_sr = BigNumber::new();
    big_num_sr.set_hex("33CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    println!("{} shirt_r 1 ->", big_num_sr.get_hex());
    big_num_sr.shift_r(1);
    println!("{}", big_num_sr.get_hex());

    //shirt_l test
    println!("shirt_l test");
    let mut big_num_sl = BigNumber::new();
    big_num_sl.set_hex("33CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    println!("{} shirt_l 1 ->", big_num_sl.get_hex());
    big_num_sl.shift_l(1);
    println!("{}", big_num_sl.get_hex());

    //add test
    println!("add test");
    
    let mut number_a_add = BigNumber::new();
    let mut number_b_add = BigNumber::new();
    
    number_a_add.set_hex("36F028580BB02CC8272A9A020F4200E346E276AE664E45EE80745574E2F5AB80");
    number_b_add.set_hex("70983D692F648185FEBE6D6FA607630AE68649F7E6FC45B94680096C06E4FADB");
    println!("{} ADD {} ->", number_a_add.get_hex(),number_b_add.get_hex() );
    number_a_add.add(&number_b_add);

    let expected_result_add = "A78865C13B14AE4E25E90771B54963EE2D68C0A64D4A8BA7C6F45EE0E9DAA65B";
    
    println!("{}", number_a_add.get_hex());
    assert_eq!(number_a_add.get_hex(), expected_result_add);

    //substarct test
    println!("substarct test");
    
    let mut number_a_sub = BigNumber::new();
    let mut number_b_sub = BigNumber::new();
    
    number_a_sub.set_hex("33CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    number_b_sub.set_hex("22E962951CB6CD2CE279AB0E2095825C141D48EF3CA9DABF253E38760B57FE03");
    println!("{} SUB {} ->", number_a_sub.get_hex(),number_b_sub.get_hex() );
    number_a_sub.subtract(&number_b_sub);

    let expected_result_sub = "10E570324E6FFDBC6B9C813DEC968D9BAD134BC0DBB061530934F4E59C2700B9";
    
    println!("{}", number_a_sub.get_hex());
    assert_eq!(number_a_sub.get_hex(), expected_result_sub);


   

    //modulo test
    println!("modulo test");
    
    let mut number_a_mod = BigNumber::new();
    let mut number_b_mod = BigNumber::new();
    
    number_a_mod.set_hex("75CED2C76B26CAE94E162C4C0D2C0FF7C13094B0185A3C122E732D5BA77EFEBC");
    number_b_mod.set_hex("22E962951CB6CD2CE279AB0E2095825C141D48EF3CA9DABF253E38760B57FE03");
    println!("{} MOD {} ->", number_a_mod.get_hex(),number_b_mod.get_hex() );
    number_a_mod.modulo(&number_b_mod);

    let expected_result_mod = "18B0F8F44C02C72875557DBEC754F85EF80BBB57B9B7B09ABA04F8BF360F129E";
    
    println!("{}", number_a_mod.get_hex());
    assert_eq!(number_a_mod.get_hex(), expected_result_mod);

   
}
