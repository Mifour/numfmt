use assert_cmd::prelude::*; // Add methods on commands
                            //use predicates::prelude::*; // Used for writing assertions
use exitcode;
use std::io::Write;
use std::process::{Command, Stdio}; // Run programs

use crate::numfmt;

pub const NUMFMT: &str = "numfmt";
/*
* use binarie path when the command does not use cargo_bin
* because "numfmt" is already a valid command in most linux distros
*/
pub const BIN_NUMFMT :&str = "./target/debug/numfmt";

/* ==============
*  | Unit tests |
*  ==============
*/


#[test]
fn test_is_int(){
    assert!(is_int("2"), true);
    assert!(is_int("-3"), true);
    assert!(is_int("0"), true);
    assert!(is_int("2.71"), false);
    assert!(is_int("a"), false);
}

#[test]
fn test_strick_positive_int(){
    assert!(strick_positive_int("2"), true);
    assert!(strick_positive_int("-3"), false);
    assert!(strick_positive_int("0"), true);
    assert!(strick_positive_int("2.71"), false);
    assert!(strick_positive_int("a"), false);
}

#[test]
fn test_validate_field(){
    assert!(validate_field(""), true);
    assert!(validate_field("3-"), true);
    assert!(validate_field("-7"), true);
    assert!(validate_field("6"), true);
    assert!(validate_field("a-b"), false);
    assert!(validate_field("0-j"), false); 
}

#[test]
fn test_validate_format(){
    assert!(validate_format(""), false);
    assert!(validate_format("%"), false);
    assert!(validate_format("f"), false);
    assert!(validate_format("%1f"), true);
    assert!(validate_format("%10f"), true);
    assert!(validate_format("%af"), false);
}

#[test]
fn test_validate_unit_from(){
    let inputs = vec!["auto", "si", "iec", "iec-i"];
    assert!(inputs.all(|input| validate_unit_form(input)), true);
    assert!(validate_unit_form("xxx"), false);
}

#[test]
fn test_validate_unit_to(){
    let inputs = vec!["si", "iec", "iec-i"];
    assert!(inputs.all(|input| validate_unit_to(input)), true);
    assert!(validate_unit_to("xxx"), false);
}

#[test]
fn test_validate_invalid(){
    let inputs = vec!["warn", "abort", "fail", "ignore"];
    assert!(inputs.all(|input| validate_invalid(input)), true);
    assert!(validate_invalid("xxx"), false);
}

#[test]
fn test_validate_round(){
    let inputs = vec!["up", "down", "from-zero", "towards-zero", "nearest"];
    assert!(inputs.all(|input| validate_round(input)), true);
    assert!(validate_round("xxx"), false);
}

#[test]
fn test_validate_si_suffix(){
    assert!(validate_si_suffix(&("K")), true);
    assert!(validate_si_suffix(&("")), false);
    assert!(validate_si_suffix(&("A")), false);
}

#[test]
fn test_validate_ieci_suffix(){
    assert!(validate_si_suffix(&("Ki")), true);
    assert!(validate_si_suffix(&("")), false);
    assert!(validate_si_suffix(&("k")), false);
}

#[test]
fn test_get_si_power(){
}

#[test]
fn test_get_iec_power(){
}

#[test]
fn test_get_auto_power(){
}

#[test]
fn test_to_si_power(){
}

#[test]
fn test_to_iec_power(){
}

#[test]
fn test_change_system(){
}

#[test]
fn test_get_fields(){
}

#[test]
fn test_padding(){

}

#[test]fn formatting(){
}

/* =====================
*  | Integration tests |
*  =====================
*/
fn pipe_command(
    command: &str, cmd_args: Vec<String>, prog: &str, prog_args: Vec<String>
)-> Result<String, Box<dyn std::error::Error>>{
    /*
    * A function that calls your program using a pipe as stdin and returns the stdout
    * args:
    *   command: str, the name of the command you want to call to fill the pipe 
    *   cmd_args: vec<str>, args to include with command
    *   prog: str, the name of the prog that should read the pipe
    *   prog_args: vec<str>, args to pass to prog
    */

    let cmd = Command::new(command).args(cmd_args)
            .output().expect("Failed to received value from command");
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
    return match output{
        Ok(res) => Ok(String::from_utf8_lossy(&res.stdout).to_string()),
        Err(e) => Err(Box::new(e))
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
fn basic_pipe()-> Result<(), Box<dyn std::error::Error>>{
    let stdout = pipe_command(
        "echo",
        vec!["42".to_string()],
        BIN_NUMFMT,
        vec![]
    )?;
    assert_eq!(stdout, "42\n");
    Ok(())
}
