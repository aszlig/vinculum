use unicode_segmentation::UnicodeSegmentation;

static CHARACTER_TUPLES: phf::Map<u32, (&str, &str, &str)> = phf::phf_map! {
    0u32 => ("I", "V", "X"),
    1u32 => ("X", "L", "C"),
    2u32 => ("C", "D", "I̅"),
    3u32 => ("I̅", "V̅", "X̅"),
    4u32 => ("X̅", "L̅", "C̅"),
    5u32 => ("C̅", "D̅", "M̅"),
    6u32 => ("M̅", "V̿", "X̿"),
    7u32 => ("X̿", "L̿", "C̿"),
    8u32 => ("C̿", "D̿", "M̿"),
    9u32 => ("I̲̿", "V̲̿", "X̲̿"),
    10u32 => ("X̲̿", "L̲̿", "C̲̿"),
    11u32 => ("C̲̿", "D̲̿", "M̲̿"),
    12u32 => ("I̳̿", "V̳̿", "X̳̿"),
    13u32 => ("X̳̿", "L̳̿", "C̳̿"),
    14u32 => ("C̳̿", "D̳̿", "M̳̿"),
    15u32 => ("I⃒̳̿", "V⃒̳̿", "X⃒̳̿"),
    16u32 => ("X⃒̳̿", "L⃒̳̿", "C⃒̳̿"),
    17u32 => ("C⃒̳̿", "D⃒̳̿", "M⃒̳̿"),
    18u32 => ("I⃦̳̿", "V⃦̳̿", "X⃦̳̿"),
    19u32 => ("X⃦̳̿", "L⃦̳̿", "C⃦̳̿"),
    20u32 => ("C⃦̳̿", "D⃦̳̿", "M⃦̳̿"),
};

// TODO me no like redundancy, merge this into the other ^ map somehow and get rid of either.
static GRAPHEME_VALUES: phf::Map<&str, (u32, u32)> = phf::phf_map! {
    "I" => (1,0),
    "V" => (5,0),
    "X" => (1,1),
    "L" => (5,1),
    "C" => (1,2),
    "D" => (5,2),
    "I̅" => (1,3),
    "V̅" => (5,3),
    "X̅" => (1,4),
    "L̅" => (5,4),
    "C̅" => (1,5),
    "D̅" => (5,5),
    "M̅" => (1,6),
    "V̿" => (5,6),
    "X̿" => (1,7),
    "L̿" => (5,7),
    "C̿" => (1,8),
    "D̿" => (5,8),
    "I̲̿" => (1,9),
    "V̲̿" => (5,9),
    "X̲̿" => (1,10),
    "L̲̿" => (5,10),
    "C̲̿" => (1,11),
    "D̲̿" => (5,11),
    "I̳̿" => (1,12),
    "V̳̿" => (5,12),
    "X̳̿" => (1,13),
    "L̳̿" => (5,13),
    "C̳̿" => (1,14),
    "D̳̿" => (5,14),
    "I⃒̳̿" => (1,15),
    "V⃒̳̿" => (5,15),
    "X⃒̳̿" => (1,16),
    "L⃒̳̿" => (5,16),
    "C⃒̳̿" => (1,17),
    "D⃒̳̿" => (5,17),
    "I⃦̳̿" => (1,18),
    "V⃦̳̿" => (5,18),
    "X⃦̳̿" => (1,19),
    "L⃦̳̿" => (5,19),
    "C⃦̳̿" => (1,20),
    "D⃦̳̿" => (5,20),

};

/// Returns a roman numeral in vinculum syntax for a given arabic number
///
/// # Arguments
///
/// * `input` - The arabic number to convert into a roman one.
///
/// # Examples
///
/// ```
/// let result = vinculum::arabic2vinculum(4711);
/// ```
pub fn arabic2vinculum(input: u64) -> Result<String, String> {
    if input == 0 {
        return Ok(String::new());
    }

    let mut result = String::new();
    let mut arabic = input;

    // From 1_000_000_000 to 10 in steps of powers of ten:
    for n in (1..=19).rev() {
        let divisor: u64 = 10_u64.pow(n);
        let divided: u64 = arabic / divisor;
        if divided > 0 {
            let appendix = make_vinculum_number(n, divided).unwrap();
            result.push_str(&appendix);
            arabic -= divisor * divided;
        }
    }
    if arabic > 0 {
        // arabic is a single digit number at this point
        let rest = make_vinculum_number(0, arabic).unwrap();
        result.push_str(&rest);
    }
    Ok(result)
}

/// Returns an arabic number for a roman numeral in vinculum syntax
///
/// # Arguments
///
/// * `input` - The String or &str holding the vinculum numeral
///
/// # Examples
///
/// ```
/// let result = vinculum::vinculum2arabic("I̅I̅I̅CI̅XCIX");
/// ```
pub fn vinculum2arabic<S: AsRef<str>>(input: S) -> Result<u64, String> {
    let values = input.as_ref().graphemes(true).map(value)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(values.iter().scan(None, |state, &next| {
        let prev = state.replace(next).unwrap_or(next);
        if prev < next {
            // We already added the previous value, so we need to subtract twice.
            next.checked_sub(prev)?.checked_sub(prev)
        } else {
            Some(next)
        }
    }).sum())
}

fn value(grapheme: &str) -> Result<u64, String> {

    match GRAPHEME_VALUES.get(grapheme) {
        Some(powers) => Ok(powers.0 as u64 * 10_u64.pow(powers.1)),
        None => Err(format!("Unknown grapheme {}", grapheme)),
    }
}

fn make_vinculum_number(power_ten: u32, times: u64) -> Result<String, String> {
    make_vinculum(times, *CHARACTER_TUPLES.get(&power_ten).unwrap())
}

fn make_vinculum(times: u64, chars: (&str, &str, &str)) -> Result<String, String> {
    macro_rules! vinc {
        [$($index:tt)*] => {
            Ok([$(chars.$index),*].concat())
        }
    }

    match times {
        1 => vinc![0],
        2 => vinc![0 0],
        3 => vinc![0 0 0],
        4 => vinc![0 1],
        5 => vinc![1],
        6 => vinc![1 0],
        7 => vinc![1 0 0],
        8 => vinc![1 0 0 0],
        9 => vinc![0 2],
        _ => Err(format!("Unsupported number: {}", times)),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_arabic2vinculum_single_digit() {
        assert_eq!(arabic2vinculum(0).unwrap(), "");
        assert_eq!(arabic2vinculum(1).unwrap(), "I");
        assert_eq!(arabic2vinculum(2).unwrap(), "II");
        assert_eq!(arabic2vinculum(3).unwrap(), "III");
        assert_eq!(arabic2vinculum(4).unwrap(), "IV");
        assert_eq!(arabic2vinculum(5).unwrap(), "V");
        assert_eq!(arabic2vinculum(6).unwrap(), "VI");
        assert_eq!(arabic2vinculum(7).unwrap(), "VII");
        assert_eq!(arabic2vinculum(8).unwrap(), "VIII");
        assert_eq!(arabic2vinculum(9).unwrap(), "IX");
    }

    #[test]
    fn test_arabic2vinculum_double_digit() {
        assert_eq!(arabic2vinculum(10).unwrap(), "X");
        assert_eq!(arabic2vinculum(11).unwrap(), "XI");
        assert_eq!(arabic2vinculum(12).unwrap(), "XII");
        assert_eq!(arabic2vinculum(13).unwrap(), "XIII");
        assert_eq!(arabic2vinculum(14).unwrap(), "XIV");
        assert_eq!(arabic2vinculum(15).unwrap(), "XV");
        assert_eq!(arabic2vinculum(19).unwrap(), "XIX");
        assert_eq!(arabic2vinculum(20).unwrap(), "XX");
        assert_eq!(arabic2vinculum(29).unwrap(), "XXIX");
        assert_eq!(arabic2vinculum(39).unwrap(), "XXXIX");
        assert_eq!(arabic2vinculum(40).unwrap(), "XL");
        assert_eq!(arabic2vinculum(50).unwrap(), "L");
        assert_eq!(arabic2vinculum(60).unwrap(), "LX");
    }

    #[test]
    fn test_arabic2vinculum_triple_digit() {
        assert_eq!(arabic2vinculum(100).unwrap(), "C");
        assert_eq!(arabic2vinculum(160).unwrap(), "CLX");
        assert_eq!(arabic2vinculum(200).unwrap(), "CC");
        assert_eq!(arabic2vinculum(246).unwrap(), "CCXLVI");
        assert_eq!(arabic2vinculum(207).unwrap(), "CCVII");
        assert_eq!(arabic2vinculum(300).unwrap(), "CCC");
        assert_eq!(arabic2vinculum(400).unwrap(), "CD");
        assert_eq!(arabic2vinculum(500).unwrap(), "D");
        assert_eq!(arabic2vinculum(600).unwrap(), "DC");
        assert_eq!(arabic2vinculum(800).unwrap(), "DCCC");
        assert_eq!(arabic2vinculum(900).unwrap(), "CI̅");
        assert_eq!(arabic2vinculum(789).unwrap(), "DCCLXXXIX");
    }

    #[test]
    fn test_arabic2vinculum_quadruple_digit() {
        assert_eq!(arabic2vinculum(1000).unwrap(), "I̅");
        assert_eq!(arabic2vinculum(1009).unwrap(), "I̅IX");
        assert_eq!(arabic2vinculum(1066).unwrap(), "I̅LXVI");
        assert_eq!(arabic2vinculum(1776).unwrap(), "I̅DCCLXXVI");
        assert_eq!(arabic2vinculum(1918).unwrap(), "I̅CI̅XVIII");
        assert_eq!(arabic2vinculum(1954).unwrap(), "I̅CI̅LIV");
        assert_eq!(arabic2vinculum(2014).unwrap(), "I̅I̅XIV");
        assert_eq!(arabic2vinculum(2421).unwrap(), "I̅I̅CDXXI");
        assert_eq!(arabic2vinculum(3999).unwrap(), "I̅I̅I̅CI̅XCIX");
        assert_eq!(arabic2vinculum(4000).unwrap(), "I̅V̅");
        assert_eq!(arabic2vinculum(4627).unwrap(), "I̅V̅DCXXVII");
        assert_eq!(arabic2vinculum(5000).unwrap(), "V̅");
        assert_eq!(arabic2vinculum(5015).unwrap(), "V̅XV");
        assert_eq!(arabic2vinculum(6000).unwrap(), "V̅I̅");
    }

    #[test]
    fn test_arabic2vinculum_quintuple_digit() {
        assert_eq!(arabic2vinculum(10000).unwrap(), "X̅");
        assert_eq!(arabic2vinculum(18034).unwrap(), "X̅V̅I̅I̅I̅XXXIV");
        assert_eq!(arabic2vinculum(20000).unwrap(), "X̅X̅");
        assert_eq!(arabic2vinculum(25000).unwrap(), "X̅X̅V̅");
        assert_eq!(arabic2vinculum(25459).unwrap(), "X̅X̅V̅CDLIX");
        assert_eq!(arabic2vinculum(50000).unwrap(), "L̅");
    }

    #[test]
    fn test_arabic2vinculum_chonky_bois() {
        assert_eq!(arabic2vinculum(100000).unwrap(), "C̅");
        assert_eq!(arabic2vinculum(500000).unwrap(), "D̅");
        assert_eq!(arabic2vinculum(500001).unwrap(), "D̅I");
        assert_eq!(arabic2vinculum(1000000).unwrap(), "M̅");
        assert_eq!(arabic2vinculum(1000001).unwrap(), "M̅I");
        assert_eq!(arabic2vinculum(2000000).unwrap(), "M̅M̅");
        assert_eq!(arabic2vinculum(3000000).unwrap(), "M̅M̅M̅");
    }

    #[test]
    fn test_arabic2vinculum_double_vinculum() {
        assert_eq!(arabic2vinculum(5000000).unwrap(), "V̿");
        assert_eq!(arabic2vinculum(10000000).unwrap(), "X̿");
        assert_eq!(arabic2vinculum(50000000).unwrap(), "L̿");
        assert_eq!(arabic2vinculum(100000000).unwrap(), "C̿");
        assert_eq!(arabic2vinculum(500000000).unwrap(), "D̿");
        // assert_eq!(arabic2vinculum(1000000000).unwrap(), "M̿"); TODO come up with a rule on
        // when to use M or the ^I in the class above
    }

    #[test]
    fn test_arabic2vinculum_irregular_numbers() {
        // for numbers which aren't actually valid roman numbers,
        // not even by vinculum's standards LOL
        // TODO add test cases for really large numbers
        assert_eq!(
            arabic2vinculum(18446744073709551615).unwrap(),
            "X⃦̳̿V⃦̳̿I⃦̳̿I⃦̳̿I⃦̳̿C⃒̳̿D⃒̳̿X⃒̳̿L⃒̳̿V⃒̳̿I⃒̳̿D̳̿C̳̿C̳̿X̳̿L̳̿I̳̿V̳̿L̲̿X̲̿X̲̿I̲̿I̲̿I̲̿D̿C̿C̿M̅X̿D̅L̅I̅DCXV"
        );
    }

    #[test]
    fn test_vinculum2arabic_single_digit() {
        assert_eq!(vinculum2arabic("I").unwrap(), 1);
        assert_eq!(vinculum2arabic("II").unwrap(), 2);
        assert_eq!(vinculum2arabic("III").unwrap(), 3);
        assert_eq!(vinculum2arabic("IV").unwrap(), 4);
        assert_eq!(vinculum2arabic("V").unwrap(), 5);
        assert_eq!(vinculum2arabic("VI").unwrap(), 6);
        assert_eq!(vinculum2arabic("VII").unwrap(), 7);
        assert_eq!(vinculum2arabic("VIII").unwrap(), 8);
        assert_eq!(vinculum2arabic("IX").unwrap(), 9);
    }

    #[test]
    fn test_vinculum2arabic_double_digit() {
        assert_eq!(vinculum2arabic("X").unwrap(), 10);
        assert_eq!(vinculum2arabic("XI").unwrap(), 11);
        assert_eq!(vinculum2arabic("XII").unwrap(), 12);
        assert_eq!(vinculum2arabic("XIII").unwrap(), 13);
        assert_eq!(vinculum2arabic("XIV").unwrap(), 14);
        assert_eq!(vinculum2arabic("XV").unwrap(), 15);
        assert_eq!(vinculum2arabic("XIX").unwrap(), 19);
        assert_eq!(vinculum2arabic("XX").unwrap(), 20);
        assert_eq!(vinculum2arabic("XXIX").unwrap(), 29);
        assert_eq!(vinculum2arabic("XXXIX").unwrap(), 39);
        assert_eq!(vinculum2arabic("XL").unwrap(), 40);
        assert_eq!(vinculum2arabic("L").unwrap(), 50);
        assert_eq!(vinculum2arabic("LX").unwrap(), 60);
    }

    #[test]
    fn test_vinculum2arabic_triple_digit() {
        assert_eq!(vinculum2arabic("C").unwrap(), 100);
        assert_eq!(vinculum2arabic("CLX").unwrap(), 160);
        assert_eq!(vinculum2arabic("CC").unwrap(), 200);
        assert_eq!(vinculum2arabic("CCXLVI").unwrap(), 246);
        assert_eq!(vinculum2arabic("CCVII").unwrap(), 207);
        assert_eq!(vinculum2arabic("CCC").unwrap(), 300);
        assert_eq!(vinculum2arabic("CD").unwrap(), 400);
        assert_eq!(vinculum2arabic("D").unwrap(), 500);
        assert_eq!(vinculum2arabic("DC").unwrap(), 600);
        assert_eq!(vinculum2arabic("DCCC").unwrap(), 800);
        assert_eq!(vinculum2arabic("CI̅").unwrap(), 900);
        assert_eq!(vinculum2arabic("DCCLXXXIX").unwrap(), 789);
    }

    #[test]
    fn test_vinculum2arabic_quadruple_digit() {
        assert_eq!(vinculum2arabic("I̅").unwrap(), 1000);
        assert_eq!(vinculum2arabic("I̅IX").unwrap(), 1009);
        assert_eq!(vinculum2arabic("I̅LXVI").unwrap(), 1066);
        assert_eq!(vinculum2arabic("I̅DCCLXXVI").unwrap(), 1776);
        assert_eq!(vinculum2arabic("I̅CI̅XVIII").unwrap(), 1918);
        assert_eq!(vinculum2arabic("I̅CI̅LIV").unwrap(), 1954);
        assert_eq!(vinculum2arabic("I̅I̅XIV").unwrap(), 2014);
        assert_eq!(vinculum2arabic("I̅I̅CDXXI").unwrap(), 2421);
        assert_eq!(vinculum2arabic("I̅I̅I̅CI̅XCIX").unwrap(), 3999);
        assert_eq!(vinculum2arabic("I̅V̅").unwrap(), 4000);
        assert_eq!(vinculum2arabic("I̅V̅DCXXVII").unwrap(), 4627);
        assert_eq!(vinculum2arabic("V̅").unwrap(), 5000);
        assert_eq!(vinculum2arabic("V̅XV").unwrap(), 5015);
        assert_eq!(vinculum2arabic("V̅I̅").unwrap(), 6000);
    }

    #[test]
    fn test_vinculum2arabic_quintuple_digit() {
        assert_eq!(vinculum2arabic("X̅").unwrap(), 10000);
        assert_eq!(vinculum2arabic("X̅V̅I̅I̅I̅XXXIV").unwrap(), 18034);
        assert_eq!(vinculum2arabic("X̅X̅").unwrap(), 20000);
        assert_eq!(vinculum2arabic("X̅X̅V̅").unwrap(), 25000);
        assert_eq!(vinculum2arabic("X̅X̅V̅CDLIX").unwrap(), 25459);
        assert_eq!(vinculum2arabic("L̅").unwrap(), 50000);
    }

    #[test]
    fn test_vinculum2arabic_chonky_bois() {
        assert_eq!(vinculum2arabic("C̅").unwrap(), 100000);
        assert_eq!(vinculum2arabic("D̅").unwrap(), 500000);
        assert_eq!(vinculum2arabic("D̅I").unwrap(), 500001);
        assert_eq!(vinculum2arabic("M̅").unwrap(), 1000000);
        assert_eq!(vinculum2arabic("M̅I").unwrap(), 1000001);
        assert_eq!(vinculum2arabic("M̅M̅").unwrap(), 2000000);
        assert_eq!(vinculum2arabic("M̅M̅M̅").unwrap(), 3000000);
    }
}
