extern crate clap;
use clap::{Arg, App};
use std::cmp;
use std::io::{self, BufRead};

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

fn validate_unit(s: String) -> Result<(), String> {
	match s.to_lowercase().as_str(){
		"auto" => Ok(()),
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
		_ => Err(String::from("invalid invalid mode"))
	}
}

fn validate_round(s: String) -> Result<(), String>{
	match s.to_lowercase().as_str(){
		"up"=> Ok(()),
		"down"=> Ok(()),
		"from-zero (default)"=> Ok(()),
		"towards-zero"=> Ok(()),
		"nearest"=> Ok(()),
		_ => Err(String::from("invalid round method"))
	}
}

fn main() {
	// let arg_vec = vec![
	// 	"debug", "delimiter", "field", "format", "from", "grouping", "header",
	// 	"invalid", "padding", "round", "suffix", "to", "to-unit", "zero_terminated",
	// 	"number"
	// ];
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
           .validator(validate_unit)
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
           .validator(validate_unit)
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
    	.get_matches(); //_from_safe(arg_vec).unwrap_or_else(|e| e.exit());
    	println!("{:?}", inputs);
    	println!("is_present header: {:?}", inputs.is_present("header"));
    // Retrieve the main arg NUMBER from Clap if possible else,
    // try with stdin (in case of pipe command)
    //let mut number = String::from(inputs.value_of("NUMBER").unwrap());
    //let mut number = String::new();
    let mut number = match inputs.value_of("NUMBER") {
    	Some(value) => String::from(value),
    	// TODO:
    	// FIX stdin read
    	None => io::stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>().join("\n")
	};
	println!("NUMBER:{:?}", number);
	if number.is_empty(){
		eprintln!("{}", "The <NUMBER> required arguments were not provided");
        std::process::exit(1);
	}
	// trim header lines from number if needed
    let header = inputs.value_of("header").unwrap_or("0")
    	.parse::<usize>().unwrap();
    let mut header_end:usize = 0;
    if header > 0{
    	let indices: Vec<(usize, &str)> = number.match_indices("\n").collect();
    	header_end  = indices[cmp::min(indices.len(), header)].0;
    }
    let h_text = &number[..header_end];
    if !h_text.is_empty(){
    	println!("HEADERS: {:?}", h_text);
    }
    number = number[header_end..].to_string();
    
    
    
    // convert string to number
    let mut res:i64 = number.parse::<i64>().unwrap();
    // scale to unit_size
    let unit_size = inputs.value_of("to-unit").unwrap_or("1")
    	.parse::<i64>().unwrap();
    res = res / unit_size;
    // convert to exporting format
    if inputs.is_present("grouping"){
    	println!("ToDo");
    }

    println!("FINAL: {:?}", res);
}
