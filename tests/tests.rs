use assert_cmd::prelude::*; // Add methods on commands
//use predicates::prelude::*; // Used for writing assertions
use exitcode;
use std::process::Command; // Run programs

/* ==============
*  | Unit tests |
*  ==============
*/

#[test]
fn test_exemples(){
	assert!(true);
}

/* =====================
*  | Integration tests |
*  =====================
*/

#[test]
fn missing_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("numfmt")?;

    cmd.assert()
        .failure()
        .code(exitcode::NOINPUT);

    Ok(())
}

#[test]
fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("numfmt")?;

    let prog = cmd
        .arg("42")
        .assert();

    prog.success()
        .stdout("42\n");

    Ok(())
}