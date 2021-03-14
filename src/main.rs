extern crate clap;
use clap::{Arg, App};

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
           .value_name("X")
           .help("change delimiter from whitespace to X")
           .takes_value(true))
    	.arg(Arg::with_name("field")
           .short("f")
           .long("field")
           .value_name("FIELDS")
           .help("replace the numbers in these input fields (default=1, see FIELDS)")
           .takes_value(true))
    	.arg(Arg::with_name("format")
           .long("format")
           .value_name("FORMAT")
           .help("use printf style floating point FORMAT (see FORMAT)")
           .takes_value(true))
    	.arg(Arg::with_name("from")
           .long("from")
           .value_name("UNIT")
           .help("specify the input unit size (default 1")
           .takes_value(true))
    	.arg(Arg::with_name("grouping")
           .long("grouping")
           .help("use locale defined grouping of digits e.g. 1,000,000 (which means it has no effect on the C/POSIX locale)"))
    	.arg(Arg::with_name("header")
           .long("header")
           .value_name("HEADERS")
           .help("print whitout convertion the first N header lins (default 1)")
           .multiple(true)
           .takes_value(true))
    	.arg(Arg::with_name("invalid")
           .long("invalid")
           .value_name("MODE")
           .help("failure mode for invalid numbers among: abort (default), fail, warn, ignore")
           .takes_value(true))
    	.arg(Arg::with_name("padding")
           .long("padding")
           .value_name("PADDING")
           .help("pad the output to N characters; positive N will right-align, negative N will left-align; padding is ignored if the output is wider than N, the defaultis is to automatically pad if a whitespace is found.")
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
           .takes_value(true))
    	.arg(Arg::with_name("to-unit")
           .long("to-unit")
           .value_name("UNIT")
           .help("the output unit size (default 1)")
           .takes_value(true))
    	.arg(Arg::with_name("zero_terminated")
           .short("z")
           .long("zero-terminated")
           .help("line delimiter is NUL, not new-line"))
    	.arg(
    		Arg::with_name("NUMBER")
    		.help("input to use")
    		.required(true))
    	.after_help(
    		"UNIT options:\n\tnone   no auto-scaling is done; suffixes will trigger an error\n\tauto   accept optional single/two letter suffix(1K = 1000, 1Ki = 1024, 1M = 1000000, 1Mi = 1048576)\n\tsi     accept optional single letter suffix(1K = 1000, 1M = 1000000)\n\tiec    accept optional single letter suffix(1K = 1024, 1M = 1048576)\n\tiec-i  accept optional two-letter suffix(1Ki = 1024, 1Mi = 1048576)
    		FIELDS supports cut(1) style field ranges:\n\tN      N'th field, counted from 1\n\n\tN-     from N'th field, to end of line\n\n\tN-M    from N'th to M'th field (inclusive)\n\n\t-M     from first to M'th field (inclusive)\n\n\t-      all fields\n\n\tMultiple fields/ranges can be separated with commas\n\n\tFORMAT must be suitable for printing one floating-point argument'%f'.  Optional quote (%'f) will enable --grouping (if supportedby current locale).  Optional width value (%10f) will pad output.Optional zero (%010f) width will zero pad the number. Optionalnegative values (%-10f) will left align.  Optional precision(%.1f) will override the input determined precision.\n\n\tExit status is 0 if all input numbers were successfullyconverted.  By default, numfmt will stop at the first conversionerror with exit status 2.  With --invalid='fail' a warning isprinted for each conversion error and the exit status is 2.  With--invalid='warn' each conversion error is diagnosed, but the exitstatus is 0.  With --invalid='ignore' conversion errors are notdiagnosed and the exit status is 0."
    		)
    	.get_matches();

    println!("{:?}", inputs.value_of("NUMBER").unwrap());
}