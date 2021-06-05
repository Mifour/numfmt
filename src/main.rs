use clap::{Arg, App, ArgMatches};
use exitcode;
use std::cmp::{max, min};
use std::io::{self, BufRead, Error, Write};


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


fn is_int(s: String) -> Result<(), String> {
	match s.parse::<i64>() {
		Ok(_) => Ok(()),
		Err(_) => Err(String::from("value should be an integer."))
	}
}

fn strick_positive_int(s: String) -> Result<(), String> {
	match s.parse::<i64>() {
		Ok(s) => match s{
			s if s > 0 => Ok(()),
			_ => Err(String::from("value should be strickly positive integer.")) 
		},
		Err(_) => Err(String::from("value should be strickly positive integer."))
	}
}

fn validate_field(s: String) -> Result<(), String>{
	let res: Vec<bool> = s.split("-")
		.into_iter()
		.map(|sub| (sub.to_owned()+"0").parse::<i64>()
			.ok()
			.map(|_| true)
		.unwrap_or(false))
		.collect();
	if res.contains(&false){
		return Err(String::from("invalid arg for field"));
	}
	Ok(())
}

fn validate_format(s: String) -> Result<(), String>{
	let start = s.find("%").unwrap_or(usize::MAX);
	if start == usize::MAX {
		return Err(String::from("invalid arg for format"));
	}
	let fstring = &s[start..];
	let stop = fstring.find("f").unwrap_or(usize::MAX);
	if stop == usize::MAX {
		return Err(String::from("invalid arg for format"));
	}
	if fstring[1..stop].is_empty(){
		return Ok(());
	}
	match fstring[1..stop].parse::<f64>() {
		Ok(_) => Ok(()),
		Err(_) => Err(String::from("invalid arg for format"))
	}
}

fn validate_unit_from(s: String) -> Result<(), String> {
	match s.to_lowercase().as_str(){
		"auto" => Ok(()),
		"si" => Ok(()),
		"iec" => Ok(()),
		"iec-i" => Ok(()),
		_ => Err(String::from("invalid unit arg"))
	}
}

fn validate_unit_to(s: String) -> Result<(), String> {
	match s.to_lowercase().as_str(){
		"si" => Ok(()),
		"iec" => Ok(()),
		"iec-i" => Ok(()),
		_ => Err(String::from("invalid unit arg"))
	}
}

fn validate_invalid(s: String) -> Result<(), String> {
	match s.to_lowercase().as_str(){
		"fail" => Ok(()),
		"warn" => Ok(()),
		"ignore" => Ok(()),
    "abort" => Ok(()),
		_ => Err(String::from("invalid invalid mode"))
	}
}

fn validate_round(s: String) -> Result<(), String>{
	match s.to_lowercase().as_str(){
		"up"=> Ok(()),
		"down"=> Ok(()),
		"from-zero"=> Ok(()),
		"towards-zero"=> Ok(()),
		"nearest"=> Ok(()),
		_ => Err(String::from("invalid round method"))
	}
}

fn validate_si_suffix(s: &String) -> bool{
	"KMGTPEZY".contains(s)
}

fn validate_ieci_suffix(s: &String) -> bool{
	"KiMiGiTiPiEiZiYi".contains(s)
}

fn get_si_power(base: &mut u32, power: &mut u32, s: &String){
  *base = 10;
	*power = match s.as_str(){
		"K" => (3),
		"M" => (6),
		"G" => (9),
		"T" => (12),
		"P" => (15),
		"E" => (18),
		"Z" => (21),
		"Y" => (24),
		_ => (1)
	};
}

fn get_iec_power(base: &mut u32, power: &mut u32, s: &String){
  *base = 2;
	*power = match s.as_str(){
		"K" | "Ki" => (10),
		"M" | "Mi" => (20),
		"G" | "Gi" => (30),
		"T" | "Ti" => (40),
		"P" | "Pi" => (50),
		"E" | "Ei" => (60),
		"Z" | "Zi" => (70),
		"Y" | "Yi" => (80),
		_ => (1)
	};
}

fn get_auto_power(base: &mut u32, power: &mut u32, s: &String){
	if validate_si_suffix(s){
		get_si_power(base, power, s);
	}
	if validate_ieci_suffix(s){
		get_iec_power(base, power, s);
	}
}

fn to_si_power(base: &u32, power: &mut u32) -> String{
	if *base == 2{
		// 2**(10*x) == 10**(3*x) for IEC standarts
		// base_2 power <=> base_10 power/10
		*power /= 10;
	}
	match *power{
		p if p >= 24 => {*power -= 24; "Y".to_string()},
		p if p >= 21 => {*power -= 21; "Z".to_string()},
		p if p >= 18 => {*power -= 18; "E".to_string()},
		p if p >= 15 => {*power -= 15; "P".to_string()},
		p if p >= 12 => {*power -= 12; "T".to_string()},
		p if p >= 9 => {*power -= 9; "G".to_string()},
		p if p >= 6 => {*power -= 6; "M".to_string()},
		p if p >= 3 => {*power -= 3; "K".to_string()},
		_ => {"".to_string()}
	}
}


fn to_iec_power(iec_i: bool, base: &u32, power: &mut u32) -> String{
	let i = match iec_i{
		true => {"i"},
		false => {""}
	};
	if *base == 10{
		// 2**(10*x) == 10**(3*x) for IEC standarts
		// base_10 power <=> base_2 10*power
		*power *= 10;
	}
	match *power{
		p if p >= 80 => {*power -= 80; "Y".to_string()+&i},
		p if p >= 70 => {*power -= 70; "Z".to_string()+&i},
		p if p >= 60 => {*power -= 60; "E".to_string()+&i},
		p if p >= 50 => {*power -= 50; "P".to_string()+&i},
		p if p >= 40 => {*power -= 40; "T".to_string()+&i},
		p if p >= 30 => {*power -= 30; "G".to_string()+&i},
		p if p >= 20 => {*power -= 20; "M".to_string()+&i},
		p if p >= 10 => {*power -= 10; "K".to_string()+&i},
		_ => {"".to_string()}
	}
}

fn change_system(from_base: &u32, to_base: &u32, power: &u32, number: &mut f64){
  let corresponding_power: u32 = match *from_base{
    2 => (*power / 10),
    _ => (*power * 10)
  };
  *number *= ((*from_base).pow(*power)/(*to_base).pow(corresponding_power)) as f64;
}

fn get_fields(fields: String) -> (usize, usize){
	match fields.find("-"){
		Some(_i) => {
			let tmp = fields.split_once("-").unwrap();
			(tmp.0.parse::<usize>().unwrap_or(usize::MAX), tmp.1.parse::<usize>().unwrap_or(usize::MAX))
		},
		_ => {(fields.parse::<usize>().unwrap_or(1), usize::MAX)}
	}
}

fn padding(res: &String, res_unit: &String, suffix: &String, n_padding: i64) -> String{
	let length = max(0, n_padding as usize - res.len() - res_unit.len());
	match n_padding{
  	i if i >= 0 =>{
  		let padding = " ".repeat(length);
  		format!("{}{}{}{}", padding, *res, *res_unit, *suffix)
  	},
  	_ =>{
  		let padding = " ".repeat(length);
  		format!("{}{}{}{}", *res, *res_unit, padding, *suffix)
  	}
  }
}

fn formatting(res: &String, res_unit: &String, suffix: &String, formatting: String) -> String{
	let start = formatting.find("%").unwrap();
	let (before,rest) = formatting.split_at(start);
	let end = rest.find("f").unwrap();
	let (format_core, after) = formatting.split_at(end+1);
	let res = padding(
		&res,
		&res_unit,
		&suffix,
		format_core.trim_start_matches("%").trim_end_matches("f").parse::<i64>().unwrap_or(1)
	);
	format!("{}{}{}", before, res, after)
}

fn numfmt_core(mut number: String, inputs: &ArgMatches, mut writer: impl std::io::Write) -> Result<String, Error>{
	let debug = inputs.is_present("debug");
  
  let mut base: u32 = 10;
  let mut power: u32 = 1;
    
  if inputs.is_present("from"){
  	let from = inputs.value_of("from").unwrap_or("auto");
  	match from.to_lowercase().as_str(){
  		"si" => {
  			get_si_power(&mut base, &mut power, &number)
  		},
  		"iec" => {
  			get_iec_power(&mut base, &mut power, &number)
  		},
  		"iec-i" => {
  			get_iec_power(&mut base, &mut power, &number)
  		}
  		_ => {
  			get_auto_power(&mut base, &mut power, &number)
  		}
  	};
  	// strip number from its old unit
  	number = "xxxx".to_string()
  }
  if debug{
  	writeln!(writer, "base:{:?}\npower:{:?}", base, power)?;
  }
    
    
    
  // convert string to number
  let mut res:f64 = number.parse::<f64>().unwrap();
  // scale to unit_size
  let unit_size = inputs.value_of("to-unit").unwrap_or("1.0")
  	.parse::<f64>().unwrap();
  res = res / unit_size;

  if inputs.is_present("rounding"){
  	match inputs.value_of("rouding").unwrap_or("from-zero").to_lowercase().as_str(){
			"up"=> {
				res = res.ceil();
			},
			"down"=> {
				res = res.floor();
			},
			"from-zero"=> {
				//away from-zero
				res = res.trunc() + res.signum();
			},
			"towards-zero"=> {
				res = res.trunc();
			},
			"nearest"=> {
				res = res.round();
			},
			_ => {}
		}
  }

  let to_base = match inputs.value_of("to").unwrap_or("si"){
    "iec" => (2),
    "si" | _ => (10)
  };
  if base != to_base{
    change_system(&base, &to_base, &power, &mut res);
  }

  let mut res_unit = "".to_string();
  if inputs.is_present("to"){
  	let to = inputs.value_of("to").unwrap();
  	res_unit = match to.to_lowercase().as_str(){
  		"si" => {
  			to_si_power(&base, &mut power)
  		},
  		"iec" => {
  			to_iec_power(false, &base, &mut power)
  		},
  		"iec-i" => {
  			to_iec_power(true, &base, &mut power)
  		}
  		_ => {res_unit}
  	};
  }
    
    
    
    
  let mut res = res.to_string();
  // convert to exporting format
  if inputs.is_present("grouping"){
  	let res_str = res.clone();
  	let (mut to_add, mut remain) = res_str.split_at(res_str.len().modulo(3));
  	let mut res_vec = vec![to_add];
  	while remain.len() > 3{
  		let x = remain.split_at(3);
  		to_add = x.0;
  		remain = x.1;
  		res_vec.push(to_add.clone());
  	}
  	res_vec.push(remain.clone());
  	res = res_vec.join(",");

  }
  let suffix =  inputs.value_of("suffix").unwrap_or("").to_string();
  if inputs.is_present("format"){

  }

  // format has higher priority because it include padding functionnalities
  let to_print = match inputs.is_present("format"){
  	true => (formatting(&res, &res_unit, &suffix, inputs.value_of("format").unwrap_or("%0f").to_string())),
  	_ => (padding(&res, &res_unit, &suffix, inputs.value_of("padding").unwrap_or("1").parse::<i64>().unwrap()))
  };
  
  if debug{
    println!("FINAL: {:?}{:?} {}", res, res_unit, suffix);
  }
  Ok(to_print)
}

fn numfmt(line: String, inputs: &ArgMatches, mut writer: impl std::io::Write) -> Result<(), Error>{
	let delimiter = inputs.value_of("delimiter").unwrap_or(" ");
	let invalid_mode = inputs.value_of("invalid").unwrap_or("abort");
	let (mut start, mut end) = get_fields(inputs.value_of("fields").unwrap_or("1").to_string());
	let vec_line: Vec<&str> = line.split(delimiter).collect();
	if start == usize::MAX{
		start = 1;
	}
	if end == usize::MAX{
		end = vec_line.len();
	}
	for number in &vec_line[(start-1)..(end-1)]{
		match invalid_mode{
			"fail" => {
        match numfmt_core(number.to_string(), &inputs, &mut writer){
          Ok(res) => writeln!(writer, "{}", res)?,
          Err(err_string) => panic!("{}", err_string)
        };
      },
			"warn" => {
        match numfmt_core(number.to_string(), &inputs, &mut writer){
          Ok(res) => writeln!(writer, "{}", res)?,
          Err(err_string) => writeln!(writer, "{}", err_string)?
        };
      },
			"ignore" => {
        match numfmt_core(number.to_string(), &inputs, &mut writer){
          Ok(res) => println!("{}", res),
          Err(_) => ()
        };
      },
      "abort" | _ => {
        match numfmt_core(number.to_string(), &inputs, &mut writer){
          Ok(res) => println!("{}", res),
          Err(_) => break
        };
      },
		};
	}
	Ok(())
}



fn main() {
	    let inputs = App::new("numfmt")
    	.version("0.1")
    	.author("mifour")
    	.about("rewrite of numfmt in rust - Convert numbers from/to human-readable strings")
    	.arg(
    		Arg::with_name("debug")
    		.long("debug")
    		.help("print warnings about invalid inputs"))
    	.arg(Arg::with_name("delimiter")
           .short("d")
           .long("delimiter")
           .value_name("DELIMITER")
           .help("change delimiter from whitespace to X")
           .takes_value(true))
    	.arg(Arg::with_name("field")
           .short("f")
           .long("field")
           .value_name("FIELD")
           .help("replace the numbers in these input fields (default=1, see FIELDS)")
           .validator(validate_field)
           .takes_value(true))
    	.arg(Arg::with_name("format")
           .long("format")
           .value_name("FORMAT")
           .help("use printf style floating point FORMAT (see FORMAT)")
           .validator(validate_format)
           .takes_value(true))
    	.arg(Arg::with_name("from")
           .long("from")
           .value_name("UNIT")
           .help("specify the input unit size (default 1")
           .validator(validate_unit_from)
           .takes_value(true))
    	.arg(Arg::with_name("grouping")
           .long("grouping")
           .help("use locale defined grouping of digits e.g. 1,000,000 (which means it has no effect on the C/POSIX locale)"))
    	.arg(Arg::with_name("header") // ToDo: fix with 0 explicit args
           .long("header")
           .value_name("N")
           .help("print whitout convertion the first N header lines (default 1)")
           .validator(strick_positive_int)
           .takes_value(true))
    	.arg(Arg::with_name("invalid")
           .long("invalid")
           .value_name("MODE")
           .help("failure mode for invalid numbers among: abort (default), fail, warn, ignore")
           .validator(validate_invalid)
           .takes_value(true))
    	.arg(Arg::with_name("padding")
           .long("padding")
           .value_name("N")
           .help("pad the output to N characters; positive N will right-align, negative N will left-align; padding is ignored if the output is wider than N, the default is to automatically pad if a whitespace is found.")
           .validator(is_int)
           .takes_value(true))
    	.arg(Arg::with_name("round")
           .long("round")
           .value_name("METHOD")
           .help("ues METHOD for rounding when scaling among up, down, from-zero (default), towards-zero, nearest")
           .validator(validate_round)
           .takes_value(true))
    	.arg(Arg::with_name("suffix")
           .long("suffix")
           .value_name("SUFFIX")
           .help("add SUFFIX to ouput numbers, and accept optionnal SUFFIX in input numbers")
           .takes_value(true))
    	.arg(Arg::with_name("to")
           .long("to")
           .value_name("UNIT")
           .help("auto scale output to UNITs (see UNITs)")
           .validator(validate_unit_to)
           .takes_value(true))
    	.arg(Arg::with_name("to-unit")
           .long("to-unit")
           .value_name("UNIT_SIZE")
           .validator(strick_positive_int)
           .help("the output unit size (default 1)")
           .takes_value(true))
    	.arg(Arg::with_name("zero_terminated")
           .short("z")
           .long("zero-terminated")
           .help("line delimiter is NUL, not new-line"))
    	.arg(
    		Arg::with_name("NUMBER")
    		.help("input to use"))
    		//.required(true))
    	.after_help(
    		"UNIT options:\n\tnone   no auto-scaling is done; suffixes will trigger an error
\tauto   accept optional single/two letter suffix(1K = 1000, 1Ki = 1024, 1M = 1000000, 1Mi = 1048576)
\tsi     accept optional single letter suffix(1K = 1000, 1M = 1000000)
\tiec    accept optional single letter suffix(1K = 1024, 1M = 1048576)
\tiec-i  accept optional two-letter suffix(1Ki = 1024, 1Mi = 1048576)
\nFIELDS supports cut(1) style field ranges:
\tN      N'th field, counted from 1
\tN-     from N'th field, to end of line
\tN-M    from N'th to M'th field (inclusive)
\t-M     from first to N'th field (inclusive)
\t-      all fields
\nMultiple fields/ranges can be separated with commas
\nFORMAT must be suitable for printing one floating-point argument'%f'.
\tOptional quote (%'f) will enable --grouping (if supported by current locale).
\tOptional width value (%10f) will pad output.
\tOptional zero (%010f) width will zero pad the number.
\tOptional negative values (%-10f) will left align.
\tOptional precision(%.1f) will override the input determined precision.
\nExit status is 0 if all input numbers were successfully converted.
\tBy default, numfmt will stop at the first conversion error with exit status 2.
\tWith --invalid='fail' a warning is printed for each conversion error and the exit status is 2.  
\tWith --invalid='warn' each conversion error is diagnosed, but the exit status is 0.  
\tWith --invalid='ignore' conversion errors are not diagnosed and the exit status is 0.
\nExamples:
\t$ numfmt --to=si 1000\n\t\t -> \"1.0K\"
\t$ numfmt --to=iec 2048\n\t\t -> \"2.0K\"
\t$ numfmt --to=iec-i 4096\n\t\t -> \"4.0Ki\"
\t$ echo 1K | numfmt --from=si\n\t\t -> \"1000\"
\t$ echo 1K | numfmt --from=iec\n\t\t -> \"1024\"
\t$ df -B1 | numfmt --header --field 2-4 --to=si
\t$ ls -l  | numfmt --header --field 5 --to=iec
\t$ ls -lh | numfmt --header --field 5 --from=iec --padding=10
\t$ ls -lh | numfmt --header --field 5 --from=iec --format %10f")
    .get_matches();
  println!("{:?}", inputs);

  /*
  ToDOs:
  1- No println, go writeln
  2- exitcode
  3- units tests
  4- integration tests
  */

  let mut writer = std::io::stdout();

  // Retrieve the main arg NUMBER from Clap if possible else,
  // try with stdin (in case of pipe command)
  let mut numbers = match inputs.value_of("NUMBER") {
  	Some(value) => String::from(value),
  	None => io::stdin().lock().lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect::<Vec<String>>().join("\n")
	};
  println!("NUMBER:{:?}", numbers);

	if numbers.is_empty(){
		eprintln!("{}", "The <NUMBER> required arguments were not provided");
        std::process::exit(exitcode::NOINPUT);
	}

  //headers
  let header = inputs.value_of("header").unwrap_or("0")
    .parse::<usize>().unwrap();
  let mut header_end:usize = 0;
  if header > 0{
    let indices: Vec<(usize, &str)> = numbers.match_indices("\n").collect();
    header_end  = indices[min(indices.len(), header)].0;
  }
  let h_text = &numbers[..header_end];
  if !h_text.is_empty(){
    match writeln!(writer,"{}", h_text){
    	Ok(_) => (),
			Err(_) => std::process::exit(exitcode::IOERR),
		};
  }
  numbers = numbers[header_end..].to_string();

  if inputs.is_present("zero_terminated"){
    numbers = numbers.replace("\0", "\n");
  }

  for number in numbers.lines(){
		match numfmt(number.to_string(), &inputs, &mut writer){
			Ok(_) => (),
			Err(_) => std::process::exit(exitcode::IOERR),
		};
	}
	let _ = writer.flush();
	std::process::exit(exitcode::OK);
}
