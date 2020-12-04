/* --- Part Two ---

The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!

You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:

byr valid:   2002
byr invalid: 2003

hgt valid:   60in
hgt valid:   190cm
hgt invalid: 190in
hgt invalid: 190

hcl valid:   #123abc
hcl invalid: #123abz
hcl invalid: 123abc

ecl valid:   brn
ecl invalid: wat

pid valid:   000000001
pid invalid: 0123456789

Here are some invalid passports:

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

Here are some valid passports:

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
*/
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

const INPUT: &str = include_str!("input");

fn main() {
    // passports separated by an empty line
    let passport_entries = INPUT.split("\n\n");

    let valid_passports = passport_entries
        .map(parse_passport)
        .filter(|v| *v)
        .count();

    println!("valid passports: {}", valid_passports);
}

type Verifier = dyn Fn(&str) -> bool;

const REQUIRED_FIELDS: &[(&str, &Verifier)] = &[
    ("byr", &|s| is_year_between(s, 1920, 2002)),
    ("iyr", &|s| is_year_between(s, 2010, 2020)),
    ("eyr", &|s| is_year_between(s, 2020, 2030)),
    ("hgt", &is_valid_height),
    ("hcl", &is_valid_hair_color),
    ("ecl", &is_valid_eye_color),
    ("pid", &is_valid_pid)
];

// returns whether passport is valid
fn parse_passport(description: &str) -> bool {
    lazy_static! {
        static ref FIELD_PARSER: Regex = Regex::new("(.{3}):(.*)").unwrap();
    }

    // we might want to build this statically
    let field_verifiers = REQUIRED_FIELDS.iter()
        .map(|x| *x)
        .collect::<HashMap<&str, &Verifier>>();

    let mut fields_found = REQUIRED_FIELDS.iter()
        .map(|(field_name, _)| (field_name.to_string(), false))
        .collect::<HashMap<_, _>>();

    for field in description.split_ascii_whitespace() {
        let captures: Captures = FIELD_PARSER.captures(field).unwrap();
        let field_name = &captures[1];
        let field_value = &captures[2];

        if let Some(verifier) = field_verifiers.get(field_name) {
            if verifier(field_value) {
                fields_found.insert(field_name.to_owned(), true);
            }
        }
    }

    fields_found.values().into_iter().all(|v| *v)
}

fn is_year_between(s: &str, min: i32, max: i32) -> bool {
    let year = s.parse::<i32>().expect("invalid year");

    year >= min && year <= max
}

fn is_valid_height(s: &str) -> bool {
    lazy_static! {
        static ref HEIGHT_PARSER: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    }

    let captures: Captures = match HEIGHT_PARSER.captures(s) {
        None => return false,
        Some(x) => x
    };

    let height = match captures[1].parse::<i32>() {
        Err(_) => return false,
        Ok(x) => x
    };

    let unit = &captures[2];

    if unit == "cm" {
        150 <= height && height <= 193
    } else {
        59 <= height && height <= 76
    }
}

fn is_valid_hair_color(s: &str) -> bool {
    lazy_static! {
        static ref HAIR_COLOR_PARSER: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    }

    HAIR_COLOR_PARSER.captures(s).is_some()
}

fn is_valid_eye_color(s: &str) -> bool {
    const VALID_EYE_COLORS: &[&str] = &[
        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    ];

    VALID_EYE_COLORS.iter().any(|color| *color == s)
}

fn is_valid_pid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
}
