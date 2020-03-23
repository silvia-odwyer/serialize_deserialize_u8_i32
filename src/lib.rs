pub mod s_d_u8_i32 {
    use std::convert::TryInto;

    pub fn exceeding_max_i32_threshold(_num: u64) -> bool {
        let max: u64 = i32::max_value().try_into().unwrap();
        if _num > max {
            true
        } else {
            false
        }
    }

    pub fn count_vec_items_left(_vec: &Vec<u8>) -> u64 {
        let items_left: u64 = _vec.len().try_into().unwrap();
        items_left
    }

    pub fn flush_value_to_zero(_value: u64, _position: u64, _size: u64) -> u64 {
        let new_value: u64 = _value
            - ((_value % (10_u64.pow(_position.try_into().unwrap())))
                - (_value % (10_u64.pow((_position - _size).try_into().unwrap()))));
        new_value
    }

    pub fn insert_value_at_position(
        _value: u64,
        _single_value: u64,
        _position: u64,
        _size: u64,
    ) -> u64 {
        // buffer up the single value to equal size i.e. turn 55 (two digits) into 055 (three digits) where the size is 3 etc.
        let mut string_single_value = _single_value.to_string();
        while string_single_value.len() < _size as usize {
            string_single_value = "0".to_owned() + &string_single_value;
        }
        let new_single_value: u64 = string_single_value.parse::<u64>().unwrap();
        let zeroed_value: u64 = flush_value_to_zero(_value, _position, _size);
        let new_value: u64 =
            zeroed_value + new_single_value * (10_u64.pow((_position - _size).try_into().unwrap()));
        new_value
    }

    pub fn access_value(_value: u64, _position: u64, _size: u64) -> u64 {
        let _mode: u64 = ((_value % (10_u64.pow(_position.try_into().unwrap()))) - (_value % (10_u64.pow((_position - _size).try_into().unwrap())))) / (10_u64.pow((_position - _size).try_into().unwrap()));
        _mode
    }

    pub fn serialize_u8_to_i32(u8_data: &mut Vec<u8>) -> Vec<i32> {
        let mut vec_of_i32s: Vec<i32> = Vec::new();
        // Test to see if there are too many i32s to store (we need to store the number of i32s in the first i32 so this can not exceed 2147483647)
        if exceeding_max_i32_threshold(count_vec_items_left(&u8_data).into()) == false {
            let mut items_left: u64 = count_vec_items_left(&u8_data);
            // Begin processing all of the data into i32s
            while items_left > 0 {
                // Create a placeholder i32
                let mut single_value_for_i32_vec: u64 = 1000000000;
                // See how many items we have left in the serialised Vec<u8>
                if items_left == 1 {
                    let one = &mut u8_data.remove(0);
                    //println!("One: {:?}", one);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *one as u64, 3, 3);
                    // Set the indicator to 3
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 10, 1);
                    // A single u8 stored in a single i32 will have a prefix of 3 - this is a code used in encoding/decoding
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, 0, 10, 1);
                }
                if items_left == 2 {
                    let one = &mut u8_data.remove(0);
                    //println!("One: {:?}", one);
                    let two = &mut u8_data.remove(0);
                    //println!("Two: {:?}", two);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 6, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *one as u64, 6, 3);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *two as u64, 3, 3);
                    // Set the indicator to 2
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 10, 1);
                    // When two u8s are stored in a single i32 it will have a prefix of 2 - this is a code used in encoding/decoding
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, 2, 10, 1);
                }
                if items_left >= 3 {
                    let one = &mut u8_data.remove(0);
                    //println!("One: {:?}", one);
                    let two = &mut u8_data.remove(0);
                    //println!("Two: {:?}", two);
                    let three = &mut u8_data.remove(0);
                    //println!("Three: {:?}", three);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 9, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *one as u64, 9, 3);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 6, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *two as u64, 6, 3);
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 3, 3);
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, *three as u64, 3, 3);
                    // Set the indicator to 2
                    single_value_for_i32_vec = flush_value_to_zero(single_value_for_i32_vec, 10, 1);
                    // When 3 u8s are stored in a single i32 it will have a prefix of 1 - this is a code used in encoding/decoding
                    single_value_for_i32_vec =
                        insert_value_at_position(single_value_for_i32_vec, 1, 10, 1);
                }
                // Calculate the remaining items left to process
                items_left = count_vec_items_left(&u8_data);
                if exceeding_max_i32_threshold(items_left.try_into().unwrap()) == false
                    && exceeding_max_i32_threshold(single_value_for_i32_vec.into()) == false
                {
                    // Push this new i32 to the vec_of_i32s
                    vec_of_i32s.push(single_value_for_i32_vec.try_into().unwrap());
                }
            }
        }
        vec_of_i32s
    }

    pub fn deserialize_i32_to_u8(_i32_data: &mut Vec<i32>) -> Vec<u8> {
        let mut vec_of_u8s: Vec<u8> = Vec::new();
        for single_i32_from_vec in _i32_data {
            println!("Processing: {:?}", single_i32_from_vec);
            let mode: u64 = access_value(*single_i32_from_vec as u64, 10, 1);
            println!("Mode: {:?}", mode);
            if mode == 1 {
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 9, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 6, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
            if mode == 2 {
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 6, 3)
                        .try_into()
                        .unwrap(),
                );
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
            // It is impossible for the other cases (which start with 1 or 2) to be less than or equal to 255. This will still work even if the 0000000000 -> 0000000255 gets appended to 0 -> 255
            if mode == 0 || single_i32_from_vec <= &mut 255 {
                vec_of_u8s.push(
                    access_value(*single_i32_from_vec as u64, 3, 3)
                        .try_into()
                        .unwrap(),
                );
            }
        }
        vec_of_u8s
    }
}

#[cfg(test)]
mod tests {
    use super::s_d_u8_i32;
    #[test]
    fn test_flush_3_3_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1000000123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1000000999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 3, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1000123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_6_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1000999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 6, 6);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_6_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 6, 3);
        assert_eq!(v, 1999000999);
    }
    #[test]
    fn test_flush_9_3_999() {
        let _test_single_value_for_i32_vec_000: u64 = 1999000000;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_000, 9, 3);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123123;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_123, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_flush_9_9_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::flush_value_to_zero(_test_single_value_for_i32_vec_999, 9, 9);
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_access_3_3_1() {
        let _test_single_value_for_i32_vec_000: u64 = 1009010011;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_000, 10, 1);
        assert_eq!(v, 1);
    }
    #[test]
    fn test_access_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123123;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_123, 3, 3);
        assert_eq!(v, 123);
    }
    #[test]
    fn test_access_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let v = s_d_u8_i32::access_value(_test_single_value_for_i32_vec_999, 3, 3);
        assert_eq!(v, 999);
    }
    #[test]
    fn test_insert_3_3_000() {
        let _test_single_value_for_i32_vec_000: u64 = 1000000000;
        let _single_val: u64 = 000;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_000,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1000000000);
    }
    #[test]
    fn test_insert_3_3_123() {
        let _test_single_value_for_i32_vec_123: u64 = 1123123000;
        let _single_val: u64 = 123;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_123,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1123123123);
    }
    #[test]
    fn test_insert_3_3_999() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999009;
        let _single_val: u64 = 999;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_999,
            _single_val,
            3,
            3,
        );
        assert_eq!(v, 1999999999);
    }
    #[test]
    fn test_insert_9_9_111() {
        let _test_single_value_for_i32_vec_999: u64 = 1999999999;
        let _single_val: u64 = 111;
        let v = s_d_u8_i32::insert_value_at_position(
            _test_single_value_for_i32_vec_999,
            _single_val,
            9,
            3,
        );
        assert_eq!(v, 1111999999);
    }
    #[test]
    fn test_i32_threshold_over() {
        let number: u64 = 2147483648;
        let b = s_d_u8_i32::exceeding_max_i32_threshold(number);
        assert_eq!(b, true);
    }
    #[test]
    fn test_i32_threshold_under() {
        let number: u64 = 2147483647;
        let b = s_d_u8_i32::exceeding_max_i32_threshold(number);
        assert_eq!(b, false);
    }
    #[test]
    fn test_count_vec_items_left() {
        let mut vec = Vec::with_capacity(10);
        for i in 0..10 {
            vec.push(i);
        }
        let items_left: u64 = s_d_u8_i32::count_vec_items_left(&vec);
        assert_eq!(items_left, 10);
    }
    #[test]
    fn test_serialize_u8_to_i32_one() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 1..=3 {
            vec.push(i);
        }
        // Creates
        // [1, 2, 3]

        // Expected result
        // [1001002003]
        let mut a: Vec<i32> = Vec::new();
        a.push(1001002003);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_serialize_u8_to_i32_two() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 1..=6 {
            vec.push(i);
        }
        // Creates
        // [1, 2, 3, 4, 5, 6]

        // Expected result
        // [1001002003, 1004005006]
        let mut a: Vec<i32> = Vec::new();
        a.push(1001002003);
        a.push(1004005006);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 2);
    }

    #[test]
    fn test_serialize_u8_to_i32_three() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 99..=105 {
            vec.push(i);
        }
        // Creates
        // [99, 100, 101, 102, 103, 104, 105]

        // Expected result
        // [1099100101, 1102103104, 0000000105]
        let mut a: Vec<i32> = Vec::new();
        a.push(1099100101);
        a.push(1102103104);
        a.push(0000000105);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_four() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 99..=106 {
            vec.push(i);
        }
        // Creates
        // [99, 100, 101, 102, 103, 104, 105, 106]

        // Expected result
        // [1099100101, 1102103104, 2000105106]
        let mut a: Vec<i32> = Vec::new();
        a.push(1099100101);
        a.push(1102103104);
        a.push(2000105106);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_five() {
        let mut vec: Vec<u8> = Vec::new();
        for i in 9..=16 {
            vec.push(i);
        }
        // Creates
        // [9, 10, 11, 12, 13, 14, 15, 16]

        // Expected result
        // [1009010011, 1012013014, 2000015016]
        let mut a: Vec<i32> = Vec::new();
        a.push(1009010011);
        a.push(1012013014);
        a.push(2000015016);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_serialize_u8_to_i32_six() {
        let mut vec: Vec<u8> = Vec::new();
        vec.push(1);
        // Creates
        // [1]

        // Expected result
        // [0000000001]
        let mut a: Vec<i32> = Vec::new();
        a.push(0000000001);

        // Actual result (check to see if a and v match)
        let v: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(&mut vec);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_deserialize_i32_to_u8_one() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1009010011);
        vec.push(1012013014);
        vec.push(2000015016);
        println!("vec: {:?}", vec);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(9);
        a.push(10);
        a.push(11);
        a.push(12);
        a.push(13);
        a.push(14);
        a.push(15);
        a.push(16);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(&mut vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 8);
    }

    #[test]
    fn test_deserialize_i32_to_u8_two() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(0000000001);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(1);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(&mut vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two thatboth match - success
        assert_eq!(matching, 1);
    }

    #[test]
    fn test_deserialize_i32_to_u8_three() {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1009010011);
        vec.push(1012013014);
        println!("vec: {:?}", vec);

        // Expected result
        let mut a: Vec<u8> = Vec::new();
        a.push(9);
        a.push(10);
        a.push(11);
        a.push(12);
        a.push(13);
        a.push(14);
        println!("a: {:?}", a);

        // Actual result (check to see if a and v match)
        let v: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(&mut vec);
        println!("v: {:?}", v);
        let matching = a.iter().zip(&v).filter(|&(a, v)| a == v).count();
        println!("{:?} vs {:?}", a, v);
        // There are two that both match - success
        assert_eq!(matching, 6);
    }
}
