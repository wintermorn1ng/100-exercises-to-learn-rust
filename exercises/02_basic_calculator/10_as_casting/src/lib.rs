// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.


fn get_bits(value: u32) -> u32 {
    let mut bits = 0;
    let mut temp_value = value;
    
    while temp_value > 0 {
        bits += 1;
        temp_value >>= 1;
    }
    bits
}
fn get_origin_code(value: u32) -> u32 {
    let bits = get_bits(value);
    let diff_code = ((1u64 << bits - 1) - 1) as u32;
    println!("{}", diff_code);
    let origin_code = (value ^ diff_code) + 1;
    origin_code
}

fn get_origin_value_8bits(value: u32) -> i8 {
    let v = value & ((1 << 7) - 1);
    let negative = (value & (1 << 8)) == 0;
    if negative {
        (v as i8) * -1
    } else {
        v as i8
    }
}

#[cfg(test)]
mod tests {
    use crate::get_origin_code;
    use crate::get_origin_value_8bits;


    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // The compiler is smart enough to know that the value 255 cannot fit
        // inside an i8, so it'll emit a hard error. We intentionally disable
        // this guardrail to make this (bad) conversion possible.
        // The compiler is only able to pick on this because the value is a
        // literal. If we were to use a variable, the compiler wouldn't be able to
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // You could solve this by using exactly the same expression as above,
        // but that would defeat the purpose of the exercise. Instead, use a genuine
        // `i8` value that is equivalent to `255` when converted from `u8`.
        let y: i8 = get_origin_value_8bits(get_origin_code(255));

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
