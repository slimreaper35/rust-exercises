use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rstest::*;

static FILE_CONTENT: &str = r#"
Lorem ipsum dolor sit amet, consectetuer adipiscing elit.
Etiam bibendum elit eget erat. Integer malesuada. Nulla est.
Integer imperdiet lectus quis justo. Class aptent taciti sociosqu ad litora torquent per conubia
nostra, per inceptos hymenaeos. Vivamus luctus egestas leo. Vestibulum fermentum tortor id mi.

Quisque porta. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit
laboriosam, nisi ut aliquid ex ea commodi consequatur? Proin in tellus sit amet nibh dignissim
sagittis. Mauris metus. Aenean placerat. Morbi scelerisque luctus velit. Nam quis nulla.
Etiam neque. Nullam feugiat, turpis at pulvinar vulputate, erat libero tristique tellus,
nec bibendum odio risus sit amet ante. Cum sociis natoque penatibus et magnis dis parturient
montes, nascetur ridiculus mus.

Suspendisse nisl. Nunc dapibus tortor vel mi dapibus sollicitudin.

Rychlá hnědá liška přeskakuje líného psacího psa, tento řádek je bajtově nejdelší pro testování
"#;

#[rstest]
fn file_doesnt_exist_returns_error_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;

    cmd.arg("-c")
        .arg("-m")
        .arg("file/does/not/exists/hopefully");
    cmd.assert().failure();
    Ok(())
}

#[rstest]
fn file_doesnt_exist_prints_to_stderr() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;

    cmd.arg("-c")
        .arg("-m")
        .arg("-L")
        .arg("file/does/not/exists/hopefully");
    cmd.assert()
        .failure()
        .stderr(predicate::str::ends_with("\n"));
    Ok(())
}

#[rstest]
#[case(false)]
#[case(true)]
fn default_arguments(#[case] with_stdin: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    let command = if with_stdin {
        cmd.pipe_stdin(file)?;
        &mut cmd
    } else {
        cmd.arg(file.path().to_str().unwrap())
    };

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("16 138 984")));
    Ok(())
}

#[rstest]
#[case(false)]
#[case(true)]
fn arguments_specified(#[case] with_stdin: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    let command = cmd.arg("-l").arg("-c");

    let command = if with_stdin {
        command.pipe_stdin(file)?;
        command
    } else {
        command.arg(file.path().to_str().unwrap())
    };

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("16 984")));
    Ok(())
}

#[rstest]
fn filename_printed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    cmd.arg("-l")
        .arg("-w")
        .arg("-c")
        .arg(file.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "16 138 984 {}\n",
            file.path().to_str().unwrap()
        )));
    Ok(())
}

#[rstest]
fn bytes_and_chars_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    cmd.arg("--chars")
        .arg("--bytes")
        .arg(file.path().to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "969 984 {}\n",
            file.path().to_str().unwrap()
        )));
    Ok(())
}

#[rstest]
#[case(false)]
#[case(true)]
fn max_line_length(#[case] with_stdin: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    let command = cmd.arg("--max-line-length");

    let command = if with_stdin {
        command.pipe_stdin(file)?;
        command
    } else {
        command.arg(file.path().to_str().unwrap())
    };

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("96 ")));
    Ok(())
}

#[rstest]
#[case(false)]
#[case(true)]
fn all_long_arguments(#[case] with_stdin: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    let command = cmd
        .arg("--lines")
        .arg("--words")
        .arg("--chars")
        .arg("--bytes")
        .arg("--max-line-length");

    let command = if with_stdin {
        command.pipe_stdin(file)?;
        command
    } else {
        command.arg(file.path().to_str().unwrap())
    };

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("16 138 969 984 96")));
    Ok(())
}

#[rstest]
#[case(false)]
#[case(true)]
fn all_short_arguments(#[case] with_stdin: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-wc")?;
    let file = assert_fs::NamedTempFile::new("lorem.txt")?;
    file.write_str(FILE_CONTENT)?;

    let command = cmd.arg("-l").arg("-w").arg("-m").arg("-c").arg("-L");

    let command = if with_stdin {
        command.pipe_stdin(file)?;
        command
    } else {
        command.arg(file.path().to_str().unwrap())
    };

    command
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("16 138 969 984 96")));
    Ok(())
}
