use core::cmp::min;
use std::error::Error;

use clap::ArgMatches;
use regex::Regex;

pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { usize i8 i16 i32 i64 }


pub const DIGITS: &str = "0123456789";
pub const DIGITALS: &str = "0123456789.,";
pub const IEC_SUFFIXES: [&'static str; 9] = ["","Ki","Mi","Gi","Ti","Pi","Ei","Zi","Yi"];
pub const SI_SUFFIXES: [&'static str; 9] = ["","K","M","G","T","P","E","Z","Y"];
pub const DEFAULT_FIELD: &str = "1-18446744073709551615";

pub fn is_int(s: String) -> Result<(), String> {
    match s.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("value should be an integer.")),
    }
}

pub fn strick_positive_int(s: String) -> Result<(), String> {
    match s.parse::<i64>() {
        Ok(s) => match s {
            s if s > 0 => Ok(()),
            _ => Err(String::from("value should be strickly positive integer.")),
        },
        Err(_) => Err(String::from("value should be strickly positive integer.")),
    }
}

pub fn validate_field(s: String) -> Result<(), String> {
    let mut res = s
        .split("-")
        .into_iter()
        .all(|sub| sub.chars().all(|c| DIGITS.contains(c)));
    res &= s.split("-").count() <= 2;
    match res {
        true => Ok(()),
        false => Err(String::from("invalid arg for field")),
    }
}

pub fn validate_format(s: String) -> Result<(), String> {
    let start = s.find("%").unwrap_or(usize::MAX);
    if start == usize::MAX {
        return Err(String::from("invalid arg for format"));
    }
    let fstring = &s[start..];
    let stop = fstring.find("f").unwrap_or(usize::MAX);
    if stop == usize::MAX {
        return Err(String::from("invalid arg for format"));
    }
    if fstring[1..stop].is_empty() {
        return Ok(());
    }
    match fstring[1..stop].parse::<f64>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("invalid arg for format")),
    }
}

pub fn validate_unit_from(s: String) -> Result<(), String> {
    match s.to_lowercase().as_str() {
        "auto" => Ok(()),
        "si" => Ok(()),
        "iec" => Ok(()),
        "iec-i" => Ok(()),
        _ => Err(String::from("invalid unit arg")),
    }
}

pub fn validate_unit_to(s: String) -> Result<(), String> {
    match s.to_lowercase().as_str() {
        "si" => Ok(()),
        "iec" => Ok(()),
        "iec-i" => Ok(()),
        _ => Err(String::from("invalid unit arg")),
    }
}

pub fn validate_invalid(s: String) -> Result<(), String> {
    match s.to_lowercase().as_str() {
        "fail" => Ok(()),
        "warn" => Ok(()),
        "ignore" => Ok(()),
        "abort" => Ok(()),
        _ => Err(String::from("invalid invalid mode")),
    }
}

pub fn validate_round(s: String) -> Result<(), String> {
    match s.to_lowercase().as_str() {
        "up" => Ok(()),
        "down" => Ok(()),
        "from-zero" => Ok(()),
        "towards-zero" => Ok(()),
        "nearest" => Ok(()),
        _ => Err(String::from("invalid round method")),
    }
}

pub fn validate_si_suffix(s: &String) -> bool {
    SI_SUFFIXES.contains(&(s.as_str()))
}

pub fn validate_ieci_suffix(s: &String) -> bool {
    IEC_SUFFIXES.contains(&(s.as_str()))
}

pub fn get_si_power(base: &mut u32, power: &mut u32, number: &mut f64, unit: &String) {
    *base = 10;
    *power = match unit.as_str() {
        "K" => (3),
        "M" => (6),
        "G" => (9),
        "T" => (12),
        "P" => (15),
        "E" => (18),
        "Z" => (21),
        "Y" => (24),
        _ => (0),
    };
    let numb_power = number.log10().trunc() as u32;
    *power += 3*(numb_power/3);
    *number /= (*base).pow(3*(numb_power/3)) as f64;
}

pub fn get_iec_power(base: &mut u32, power: &mut u32, number: &mut f64, unit: &String) {
    *base = 2;
    *power = match unit.as_str() {
        "K" | "Ki" => (10),
        "M" | "Mi" => (20),
        "G" | "Gi" => (30),
        "T" | "Ti" => (40),
        "P" | "Pi" => (50),
        "E" | "Ei" => (60),
        "Z" | "Zi" => (70),
        "Y" | "Yi" => (80),
        _ => (0),
    };
    let numb_power = number.log2().trunc() as u32;
    *power += 10*(numb_power/10);
    *number /= (*base).pow(10*(numb_power/10)) as f64;
}

pub fn get_auto_power(base: &mut u32, power: &mut u32, number: &mut f64, s: &String) {
    if validate_si_suffix(s) {
        get_si_power(base, power, number, s);
        return;
    }
    if validate_ieci_suffix(s) {
        get_iec_power(base, power, number, s);
        return;
    }
}

pub fn to_si_power(base: &u32, power: &mut u32) -> String {
    if *base == 2 {
        // 2**(10*x) == 10**(3*x) for IEC standarts
        // base_2 power <=> base_10 power*3/10
        *power = (*power / 10) * 3;
    }
    match *power {
        p if p >= 24 => {
            //*power -= 24;
            "Y".to_string()
        }
        p if p >= 21 => {
            //*power -= 21;
            "Z".to_string()
        }
        p if p >= 18 => {
            //*power -= 18;
            "E".to_string()
        }
        p if p >= 15 => {
            //*power -= 15;
            "P".to_string()
        }
        p if p >= 12 => {
            //*power -= 12;
            "T".to_string()
        }
        p if p >= 9 => {
            //*power -= 9;
            "G".to_string()
        }
        p if p >= 6 => {
            //*power -= 6;
            "M".to_string()
        }
        p if p >= 3 => {
            //*power -= 3;
            "K".to_string()
        }
        _ => "".to_string(),
    }
}

pub fn to_iec_power(iec_i: bool, base: &u32, power: &mut u32) -> String {
    let i = match iec_i {
        true => "i",
        false => "",
    };
    if *base == 10 {
        // 2**(10*x) == 10**(3*x) for IEC standarts
        // base_10 power <=> base_2 10*power/3
        *power = (*power * 10) / 3;
    }
    match *power {
        p if p >= 80 => {
            //*power -= 80;
            "Y".to_string() + &i
        }
        p if p >= 70 => {
            //*power -= 70;
            "Z".to_string() + &i
        }
        p if p >= 60 => {
            //*power -= 60;
            "E".to_string() + &i
        }
        p if p >= 50 => {
            //*power -= 50;
            "P".to_string() + &i
        }
        p if p >= 40 => {
            //*power -= 40;
            "T".to_string() + &i
        }
        p if p >= 30 => {
            //*power -= 30;
            "G".to_string() + &i
        }
        p if p >= 20 => {
            //*power -= 20;
            "M".to_string() + &i
        }
        p if p >= 10 => {
            //*power -= 10;
            "K".to_string() + &i
        }
        _ => "".to_string(),
    }
}

pub fn change_system(from_base: &u32, to_base: &u32, power: &u32, number: &mut f64) {
    let corresponding_power: u32 = match *from_base {
        2 => (*power / 10) * 3,
        _ => (*power * 10) / 3,
    };
    *number *= (*from_base).pow(*power) as f64 / (*to_base).pow(corresponding_power) as f64;
}

pub fn get_fields(fields: String) -> (usize, usize) {
    match fields.find("-") {
        Some(_i) => {
            let tmp = fields.split_once("-").unwrap();
            (
                tmp.0.parse::<usize>().unwrap_or(usize::MAX),
                tmp.1.parse::<usize>().unwrap_or(usize::MAX),
            )
        }
        _ => {
            match fields.parse::<usize>(){
                Ok(i) => (i,i),
                Err(_) => (1, usize::MAX)
            }
        },
    }
}

pub fn padding(res: &String, res_unit: &String, suffix: &String, n_padding: i64) -> String {
    let length = n_padding as usize - min(res.len() + res_unit.len(), n_padding as usize);
    match n_padding {
        i if i >= 0 => {
            let padding = " ".repeat(length);
            format!("{}{}{}{}", padding, *res, *res_unit, *suffix)
        }
        _ => {
            let padding = " ".repeat(length);
            format!("{}{}{}{}", *res, *res_unit, padding, *suffix)
        }
    }
}

pub fn formatting(res: &String, res_unit: &String, suffix: &String, formatting: String) -> String {
    let start = formatting.find("%").unwrap();
    let (before, rest) = formatting.split_at(start);
    let end = rest.find("f").unwrap();
    let (format_core, after) = formatting.split_at(end + 1);
    let res = padding(
        &res,
        &res_unit,
        &suffix,
        format_core
            .trim_start_matches("%")
            .trim_end_matches("f")
            .parse::<i64>()
            .unwrap_or(1),
    );
    format!("{}{}{}", before, res, after)
}

pub fn strip_number(
    number: &mut String,
    suffix: &mut String,
) -> Result<f64, std::num::ParseFloatError> {
    /*
    Try striping the number by itering on chars until it is not a digit anymore.
    */
    let mut char_indices = number.char_indices();
    let mut char_index = char_indices.next();
    loop {
        match char_index {
            Some((i, c)) => {
                if !DIGITALS.contains(c) {
                    let tmp = (*number).split_at(i);
                    *suffix = tmp.1.to_string();
                    *number = tmp.0.to_string();
                    break;
                }
                char_index = char_indices.next();
            }
            None => {
                *suffix = "".to_string();
                break;
            }
        }
    }
    return (*number).parse::<f64>();
}

pub fn numfmt_core(
    mut number: String,
    inputs: &ArgMatches,
    locale_decimal_point: &str,
    mut writer: impl std::io::Write,
) -> Result<String, Box<dyn Error>> {

    /* 
    First part of this function about "understanding" or "importing" the number.
    The second half is more about "preparing" or "exporting" the number.
    */

    let debug = inputs.is_present("debug");
    let mut base: u32 = 10;
    let mut power: u32 = 1;
    let mut suffix = String::new();

    // convert string to number
    let mut res: f64;
    number = number.replace(locale_decimal_point, ".");
    //println!("number {}", number);

    match strip_number(&mut number, &mut suffix) {
        Ok(n) => {
            res = n;
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }
    //println!("res {}", res);

    // determine base and power of number
    let from = inputs.value_of("from").unwrap_or("auto");
    match from.to_lowercase().as_str() {
        "si" => get_si_power(&mut base, &mut power, &mut res, &suffix),
        "iec" => get_iec_power(&mut base, &mut power, &mut res, &suffix),
        "iec-i" => get_iec_power(&mut base, &mut power, &mut res, &suffix),
        _ => get_auto_power(&mut base, &mut power, &mut res, &suffix),
    };
    
    if debug {
        writeln!(writer, "base:{:?}\npower:{:?}", base, power)?;
    }
    
    // scale to unit_size
    let unit_size = inputs
        .value_of("to-unit")
        .unwrap_or("1.0")
        .parse::<f64>()
        .unwrap();
    res = res / unit_size;

    if inputs.is_present("rounding") {
        match inputs
            .value_of("rouding")
            .unwrap_or("from-zero")
            .to_lowercase()
            .as_str()
        {
            "up" => {
                res = res.ceil();
            }
            "down" => {
                res = res.floor();
            }
            "from-zero" => {
                //away from-zero
                res = res.trunc() + res.signum();
            }
            "towards-zero" => {
                res = res.trunc();
            }
            "nearest" => {
                res = res.round();
            }
            _ => {}
        }
    }

    // preparing the base to export the number
    let to_base = match inputs.value_of("to").unwrap_or("si") {
        "iec" | "iec-i" => (2),
        "si" | _ => (10),
    };
    // change base only if necessary
    if base != to_base {
        change_system(&base, &to_base, &power, &mut res);
    }

    let mut res_unit = "".to_string();
    if inputs.is_present("to") {
        let to = inputs.value_of("to").unwrap();
        res_unit = match to.to_lowercase().as_str() {
            "si" => to_si_power(&base, &mut power),
            "iec" => to_iec_power(false, &base, &mut power),
            "iec-i" => to_iec_power(true, &base, &mut power),
            _ => res_unit,
        };
    }
    else{
        res_unit = suffix;
    }

    let mut res = res.to_string();
    // convert to exporting format
    if inputs.is_present("grouping") {
        let res_str = res.clone();
        let (mut to_add, mut remain) = res_str.split_at(res_str.len().modulo(3));
        let mut res_vec = vec![to_add];
        while remain.len() > 3 {
            let x = remain.split_at(3);
            to_add = x.0;
            remain = x.1;
            res_vec.push(to_add.clone());
        }
        res_vec.push(remain.clone());
        res = res_vec.join(",");
    }
    let suffix = inputs.value_of("suffix").unwrap_or("").to_string();
    if inputs.is_present("format") {}

    // format has higher priority because it include padding functionnalities
    let to_print = match inputs.is_present("format") {
        true => formatting(
            &res,
            &res_unit,
            &suffix,
            inputs.value_of("format").unwrap_or("%0f").to_string(),
        ),
        _ => padding(
            &res,
            &res_unit,
            &suffix,
            inputs
                .value_of("padding")
                .unwrap_or("1")
                .parse::<i64>()
                .unwrap(),
        ),
    };

    if debug {
        println!("FINAL: {:?}{:?} {}", res, res_unit, suffix);
        println!("or:{}",to_print);
    }
    Ok(to_print)
}

pub fn numfmt(
    line: String,
    inputs: &ArgMatches,
    locale_decimal_point: &str,
    mut writer: impl std::io::Write,
) -> Result<(), Box<dyn Error>> {
    let delimiter = inputs.value_of("delimiter").unwrap_or(" ");
    let invalid_mode = inputs.value_of("invalid").unwrap_or("fail"); //default is abort
    let (mut start, end) = get_fields(inputs.value_of("field").unwrap_or(&DEFAULT_FIELD).to_string());
    if start == usize::MAX {
        start = 1;
    }
    //println!("fields {},{}", start, end);

    let re = Regex::new(
        &(format!(r"([^{}]+|[{}]+)", delimiter, delimiter).as_str())
    ).unwrap();

    let mut index = 1;
    let mut space: bool;
    for cap in re.captures_iter(&line) {
        // iter on each group of chars
        let field = &cap[0];
        space = field.contains(delimiter);
        if !space{
            index += 1;
        }
        //println!("re line: {} at {}", field, index);
        if !space && start <= index && index <= end{
            // if the group of char is one of those asked by the user, format it
            match invalid_mode {
                "fail" => {
                    match numfmt_core(field.to_string(), &inputs, &locale_decimal_point, &mut writer) {
                        Ok(res) => write!(writer, "{}", res)?,
                        Err(err_string) => {
                            return Err(err_string);
                        }
                    };
                }
                "warn" => {
                    match numfmt_core(field.to_string(), &inputs, &locale_decimal_point, &mut writer) {
                        Ok(res) => write!(writer, "{}", res)?,
                        Err(err_string) => write!(writer, "{}", err_string)?,
                    };
                }
                "ignore" => {
                    match numfmt_core(field.to_string(), &inputs, &locale_decimal_point, &mut writer) {
                        Ok(res) => write!(writer, "{}", res)?,
                        Err(_) => (),
                    };
                }
                "abort" | _ => {
                    match numfmt_core(field.to_string(), &inputs, &locale_decimal_point, &mut writer) {
                        Ok(res) => write!(writer, "{}", res)?,
                        Err(_) => break,
                    };
                }
            };
        }
        else{
            write!(writer, "{}", field)?;
        }
    }
    // write newline
    Ok(write!(writer, "{}", if inputs.is_present("zero_terminated") {"\0"} else {"\n"})?)
}
