use assert_cmd::prelude::*; // Add methods on commands
                            //use predicates::prelude::*; // Used for writing assertions
use exitcode;
use std::io::Write;
use std::process::{Command, Stdio}; // Run programs

use numfmt::*;

pub const NUMFMT: &str = "numfmt";
/*
* use binary path when the command does not use cargo_bin
* because "numfmt" is already a valid command in most linux distros
*/
pub const BIN_NUMFMT: &str = "./target/debug/numfmt";

/* ==============
*  | Unit tests |
*  ==============
*/

#[test]
fn test_is_int() {
    assert_eq!(is_int("2".to_string()), Ok(()));
    assert_eq!(is_int("-3".to_string()), Ok(()));
    assert_eq!(is_int("0".to_string()), Ok(()));
    assert_ne!(is_int("2.71".to_string()), Ok(()));
    assert_ne!(is_int("a".to_string()), Ok(()));
}

#[test]
fn test_strick_positive_int() {
    assert_eq!(strick_positive_int("2".to_string()), Ok(()));
    assert_ne!(strick_positive_int("0".to_string()), Ok(()));
    assert_ne!(strick_positive_int("-3".to_string()), Ok(()));
    assert_ne!(strick_positive_int("2.71".to_string()), Ok(()));
    assert_ne!(strick_positive_int("a".to_string()), Ok(()));
}

#[test]
fn test_validate_field() {
    assert_eq!(validate_field("".to_string()), Ok(()));
    assert_eq!(validate_field("3-".to_string()), Ok(()));
    assert_eq!(validate_field("-7".to_string()), Ok(()));
    assert_eq!(validate_field("6".to_string()), Ok(()));
    assert_ne!(validate_field("a-b".to_string()), Ok(()));
    assert_ne!(validate_field("0-j".to_string()), Ok(()));
}

#[test]
fn test_validate_format() {
    assert_eq!(validate_format("%1f".to_string()), Ok(()));
    assert_eq!(validate_format("%10f".to_string()), Ok(()));
    assert_ne!(validate_format("".to_string()), Ok(()));
    assert_ne!(validate_format("%".to_string()), Ok(()));
    assert_ne!(validate_format("f".to_string()), Ok(()));
    assert_ne!(validate_format("%af".to_string()), Ok(()));
}

#[test]
fn test_validate_unit_from() {
    let inputs = vec!["auto", "si", "iec", "iec-i"];
    assert!(inputs
        .iter()
        .all(|input| validate_unit_from(input.to_string()) == Ok(())));
    assert_ne!(validate_unit_from("xxx".to_string()), Ok(()));
}

#[test]
fn test_validate_unit_to() {
    let inputs = vec!["si", "iec", "iec-i"];
    assert!(inputs
        .iter()
        .all(|input| validate_unit_to(input.to_string()) == Ok(())));
    assert_ne!(validate_unit_to("xxx".to_string()), Ok(()));
}

#[test]
fn test_validate_invalid() {
    let inputs = vec!["warn", "abort", "fail", "ignore"];
    assert!(inputs
        .iter()
        .all(|input| validate_invalid(input.to_string()) == Ok(())));
    assert_ne!(validate_invalid("xxx".to_string()), Ok(()));
}

#[test]
fn test_validate_round() {
    let inputs = vec!["up", "down", "from-zero", "towards-zero", "nearest"];
    assert!(inputs
        .iter()
        .all(|input| validate_round(input.to_string()) == Ok(())));
    assert_ne!(validate_round("xxx".to_string()), Ok(()));
}

#[test]
fn test_validate_si_suffix() {
    assert_eq!(validate_si_suffix(&"K".to_string()), true);
    assert_eq!(validate_si_suffix(&"".to_string()), false);
    assert_eq!(validate_si_suffix(&"A".to_string()), false);
}

#[test]
fn test_validate_ieci_suffix() {
    assert_eq!(validate_ieci_suffix(&"Ki".to_string()), true);
    assert_eq!(validate_ieci_suffix(&"".to_string()), false);
    assert_eq!(validate_ieci_suffix(&"k".to_string()), false);
}

#[test]
fn test_get_si_power() {
    let mut base = 10;
    let mut power = 1;
    let s = "T".to_string();
    get_si_power(&mut base, &mut power, &s);
    assert_eq!((base, power), (10, 12));
}

#[test]
fn test_get_iec_power() {
    let mut base = 2;
    let mut power = 1;
    let s = "Pi".to_string();
    get_iec_power(&mut base, &mut power, &s);
    assert_eq!((base, power), (2, 50));
}

#[test]
fn test_get_auto_power() {
    let mut base = 2;
    let mut power = 1;
    let s = "T".to_string();
    get_si_power(&mut base, &mut power, &s);
    assert_eq!((base, power), (10, 12));

    let mut base = 2;
    let mut power = 1;
    let s = "Pi".to_string();
    get_iec_power(&mut base, &mut power, &s);
    assert_eq!((base, power), (2, 50));
}

#[test]
fn test_to_si_power() {
    let base = 2;
    let mut power = 50;
    let suffix = to_si_power(&base, &mut power);
    assert_eq!((power, suffix), (15, "P".to_string()));
}

#[test]
fn test_to_iec_power() {
    let base = 10;
    let mut power = 12;
    let suffix = to_iec_power(true, &base, &mut power);
    assert_eq!((power, suffix), (40, "Ti".to_string()));
}

#[test]
fn test_change_system() {
    let mut number = 2048.0;
    change_system(&10, &2, &3, &mut number);
    assert_eq!(number, 2000.0);
}

#[test]
fn test_get_fields() {
    assert_eq!(get_fields("1".to_string()), (1, usize::MAX));
    assert_eq!(get_fields("a".to_string()), (1, usize::MAX));
    assert_eq!(get_fields("a-".to_string()), (usize::MAX, usize::MAX));
    assert_eq!(get_fields("".to_string()), (1, usize::MAX));
    assert_eq!(get_fields("0-".to_string()), (0, usize::MAX));
    assert_eq!(get_fields("-10".to_string()), (usize::MAX, 10));
    assert_eq!(get_fields("0-1".to_string()), (0, 1));
}

#[test]
fn test_padding() {
    assert_eq!(
        padding(&"64".to_string(), &"Ki".to_string(), &"".to_string(), 8),
        "    64Ki".to_string()
    );
    assert_eq!(
        padding(
            &"480".to_string(),
            &"M".to_string(),
            &" cookies".to_string(),
            4
        ),
        "480M cookies".to_string()
    );
}

#[test]
fn test_formatting() {
    assert_eq!(
        formatting(
            &"64".to_string(),
            &"Ki".to_string(),
            &"".to_string(),
            "%8f".to_string()
        ),
        "    64Ki".to_string()
    );
    assert_eq!(
        formatting(
            &"480".to_string(),
            &"M".to_string(),
            &" cookies".to_string(),
            "%f".to_string()
        ),
        "480M cookies".to_string()
    );
}

/* =====================
*  | Integration tests |
*  =====================
*/
fn pipe_command(
    command: &str,
    cmd_args: Vec<String>,
    prog: &str,
    prog_args: Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    /*
     * A function that calls your program using a pipe as stdin and returns the stdout
     * args:
     *   command: str, the name of the command you want to call to fill the pipe
     *   cmd_args: vec<str>, args to include with command
     *   prog: str, the name of the prog that should read the pipe
     *   prog_args: vec<str>, args to pass to prog
     */

    let cmd = Command::new(command)
        .args(cmd_args)
        .output()
        .expect("Failed to received value from command");
    let pipe = cmd.stdout;
    println!("First program output: {:?}", pipe);
    let mut child = Command::new(prog)
        .args(prog_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to lauch process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        let _ = stdin.write_all(&pipe[..]);
    });

    let output = child.wait_with_output();
    return match output {
        Ok(res) => Ok(String::from_utf8_lossy(&res.stdout).to_string()),
        Err(e) => Err(Box::new(e)),
    };
}

#[test]
fn missing_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    cmd.assert().failure().code(exitcode::NOINPUT);
    Ok(())
}

#[test]
fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    let prog = cmd.arg("42").assert();
    prog.success().stdout("42\n");
    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    let prog = cmd.arg("--help").assert();
    prog.success();
    Ok(())
}

#[test]
fn basic_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = pipe_command(
        "echo",
        vec!["42".to_string()],
        BIN_NUMFMT,
        vec!["--padding=8".to_string()],
    )?;
    assert_eq!(stdout, "      42\n");
    Ok(())
}

#[test]
fn test_example0() -> Result<(), Box<dyn std::error::Error>> {
    //numfmt --to=si 1000 -> "1.0K"
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    let prog = cmd.args(&["--to=si", "1000"]).assert();
    prog.success().stdout("1.0K\n");
    Ok(())
}

#[test]
fn test_example1() -> Result<(), Box<dyn std::error::Error>> {
    //numfmt --to=iec 2048 -> "2.0K"
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    let prog = cmd.args(&["--to=iec", "2048"]).assert();
    prog.success().stdout("2.0K\n");
    Ok(())
}

#[test]
fn test_example2() -> Result<(), Box<dyn std::error::Error>> {
    //numfmt --to=iec-i 4096 -> "4.0Ki"
    let mut cmd = Command::cargo_bin(NUMFMT)?;
    let prog = cmd.args(&["--to=iec-i", "4096"]).assert();
    prog.success().stdout("4.0Ki\n");
    Ok(())
}

#[test]
fn test_example3() -> Result<(), Box<dyn std::error::Error>> {
    //echo 1K | numfmt --from=si -> "1000"
    let stdout = pipe_command("echo", vec!["1K".to_string()], BIN_NUMFMT, vec![])?;
    assert_eq!(stdout, "1000\n");
    Ok(())
}

#[test]
fn test_example4() -> Result<(), Box<dyn std::error::Error>> {
    //echo 1K | numfmt --from=iec -> "1024"
    let stdout = pipe_command(
        "echo",
        vec!["1K".to_string()],
        BIN_NUMFMT,
        vec!["--from=iec".to_string()],
    )?;
    assert_eq!(stdout, "1024\n");
    Ok(())
}

#[test]
fn test_example5() -> Result<(), Box<dyn std::error::Error>> {
    //df -B1 | numfmt --header --field 2-4 --to=si
    let stdout = pipe_command(
        "cat",
        vec!["./tests/df_dump.txt".to_string()],
        BIN_NUMFMT,
        vec![
            "--header=1".to_string(),
            "--field=2-4".to_string(),
            "--to=si".to_string(),
        ],
    )?;
    assert_eq!(stdout, "");
    Ok(())
}

#[test]
fn test_example6() -> Result<(), Box<dyn std::error::Error>> {
    //ls -l  | numfmt --header --field 5 --to=iec
    let stdout = pipe_command(
        "cat",
        vec!["./tests/ls_l_dump.txt".to_string()],
        BIN_NUMFMT,
        vec![
            "--header=1".to_string(),
            "--field=5".to_string(),
            "--to=iec".to_string(),
        ],
    )?;
    assert_eq!(stdout, "");
    Ok(())
}

#[test]
fn test_example7() -> Result<(), Box<dyn std::error::Error>> {
    //ls -lh | numfmt --header --field 5 --from=iec --padding=10
    let stdout = pipe_command(
        "cat",
        vec!["./tests/ls_lh_dump.txt".to_string()],
        BIN_NUMFMT,
        vec![
            "--header=1".to_string(),
            "--field=5".to_string(),
            "--from=iec".to_string(),
            "--padding=10".to_string(),
        ],
    )?;
    assert_eq!(stdout, "");
    Ok(())
}

#[test]
fn test_example8() -> Result<(), Box<dyn std::error::Error>> {
    //ls -lh | numfmt --header --field 5 --format=%10f
    let stdout = pipe_command(
        "cat",
        vec!["./tests/ls_lh_dump.txt".to_string()],
        BIN_NUMFMT,
        vec![
            "--header=1".to_string(),
            "--field=5".to_string(),
            "--from=iec".to_string(),
            "--format=%10f".to_string(),
        ],
    )?;
    assert_eq!(stdout, "");
    Ok(())
}
