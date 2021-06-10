use clap::{App, Arg};
use exitcode;
use std::cmp::min;
use std::io::{self, BufRead, Write};

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
           .validator(numfmt::validate_field)
           .takes_value(true))
    	.arg(Arg::with_name("format")
           .long("format")
           .value_name("FORMAT")
           .help("use printf style floating point FORMAT (see FORMAT)")
           .validator(numfmt::validate_format)
           .takes_value(true))
    	.arg(Arg::with_name("from")
           .long("from")
           .value_name("UNIT")
           .help("specify the input unit size (default 1")
           .validator(numfmt::validate_unit_from)
           .takes_value(true))
    	.arg(Arg::with_name("grouping")
           .long("grouping")
           .help("use locale defined grouping of digits e.g. 1,000,000 (which means it has no effect on the C/POSIX locale)"))
    	.arg(Arg::with_name("header") // ToDo: fix with 0 explicit args
           .long("header")
           .value_name("N")
           .help("print whitout convertion the first N header lines (default 1)")
           .validator(numfmt::strick_positive_int)
           .takes_value(true))
    	.arg(Arg::with_name("invalid")
           .long("invalid")
           .value_name("MODE")
           .help("failure mode for invalid numbers among: abort (default), fail, warn, ignore")
           .validator(numfmt::validate_invalid)
           .takes_value(true))
    	.arg(Arg::with_name("padding")
           .long("padding")
           .value_name("N")
           .help("pad the output to N characters; positive N will right-align, negative N will left-align; padding is ignored if the output is wider than N, the default is to automatically pad if a whitespace is found.")
           .validator(numfmt::is_int)
           .takes_value(true))
    	.arg(Arg::with_name("round")
           .long("round")
           .value_name("METHOD")
           .help("ues METHOD for rounding when scaling among up, down, from-zero (default), towards-zero, nearest")
           .validator(numfmt::validate_round)
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
           .validator(numfmt::validate_unit_to)
           .takes_value(true))
    	.arg(Arg::with_name("to-unit")
           .long("to-unit")
           .value_name("UNIT_SIZE")
           .validator(numfmt::strick_positive_int)
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
        None => io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>()
            .join("\n"),
    };

    if numbers.is_empty() {
        eprintln!("{}", "The <NUMBER> required arguments were not provided");
        std::process::exit(exitcode::NOINPUT);
    }

    //headers
    let header = inputs
        .value_of("header")
        .unwrap_or("0")
        .parse::<usize>()
        .unwrap();
    let mut header_end: usize = 0;
    if header > 0 {
        let indices: Vec<(usize, &str)> = numbers.match_indices("\n").collect();
        header_end = indices[min(indices.len(), header)].0;
    }
    let h_text = &numbers[..header_end];
    if !h_text.is_empty() {
        match writeln!(writer, "{}", h_text) {
            Ok(_) => (),
            Err(_) => std::process::exit(exitcode::IOERR),
        };
    }
    numbers = numbers[header_end..].to_string();

    if inputs.is_present("zero_terminated") {
        numbers = numbers.replace("\0", "\n");
    }

    for number in numbers.lines() {
        match numfmt::numfmt(number.to_string(), &inputs, &mut writer) {
            Ok(_) => (),
            Err(_) => std::process::exit(exitcode::IOERR),
        };
    }
    let _ = writer.flush();
    std::process::exit(exitcode::OK);
}
