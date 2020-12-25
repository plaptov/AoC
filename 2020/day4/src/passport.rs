use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYES: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
lazy_static! {
    static ref YEAR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^\d{2,3}(cm|in){1}$").unwrap();
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^[\d]{9}$").unwrap();
}

pub struct Passport<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    pub fn new(lines: &'a [String]) -> Self {
        let mut data = HashMap::new();
        for line in lines {
            for part in line.split(" ") {
                let mut key_value = part.split(":");
                let key = key_value.next().unwrap();
                let value = key_value.next().unwrap();
                data.insert(key, value);
            }
        }

        Passport { data }
    }

    pub fn all_fields_presents(&self) -> bool {
        FIELDS.iter().all(|key| self.data.contains_key(key))
    }

    pub fn all_fields_valid(&self) -> bool {
        self.is_birth_year_valid()
            && self.is_issue_year_valid()
            && self.is_expiry_year_valid()
            && self.is_height_valid()
            && self.is_hair_color_valid()
            && self.is_eye_color_valid()
            && self.is_pid_valid()
    }

    fn is_birth_year_valid(&self) -> bool {
        let value = self.data.get("byr").unwrap();
        if !YEAR_REGEX.is_match(value) {
            return false;
        }
        let year = i32::from_str(value).unwrap();
        1920 <= year && year <= 2002
    }

    fn is_issue_year_valid(&self) -> bool {
        let value = self.data.get("iyr").unwrap();
        if !YEAR_REGEX.is_match(value) {
            return false;
        }
        let year = i32::from_str(value).unwrap();
        2010 <= year && year <= 2020
    }

    fn is_expiry_year_valid(&self) -> bool {
        let value = self.data.get("eyr").unwrap();
        if !YEAR_REGEX.is_match(value) {
            return false;
        }
        let year = i32::from_str(value).unwrap();
        2020 <= year && year <= 2030
    }

    fn is_height_valid(&self) -> bool {
        let value = self.data.get("hgt").unwrap();
        if !HEIGHT_REGEX.is_match(value) {
            return false;
        }
        let (val, unit) = value.split_at(value.len() - 2);
        let height = i32::from_str(val).unwrap();
        if unit == "cm" {
            150 <= height && height <= 193
        } else {
            59 <= height && height <= 76
        }
    }

    fn is_hair_color_valid(&self) -> bool {
        let value = self.data.get("hcl").unwrap();
        COLOR_REGEX.is_match(value)
    }

    fn is_eye_color_valid(&self) -> bool {
        let value = self.data.get("ecl").unwrap();
        EYES.contains(value)
    }

    fn is_pid_valid(&self) -> bool {
        let value = self.data.get("pid").unwrap();
        PID_REGEX.is_match(value)
    }
}
