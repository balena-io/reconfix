use std::{
    env,
    fs::{read_dir, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

fn main() -> Result<(), Error> {
    generate_validator_tests()?;
    Ok(())
}

fn generate_validator_tests() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join("validator_tests.rs");
    let mut test_file = File::create(&destination)?;
    generate_validator_tests_module(&mut test_file, &PathBuf::from_str("./tests/data/validator").unwrap())?;
    Ok(())
}

fn generate_validator_tests_module(mut test_file: &mut File, dir: &PathBuf) -> Result<(), Error> {
    let module_name = normalize_file_stem(dir)?;
    start_validator_module(&mut test_file, &module_name)?;

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path().canonicalize()?;

        if path.is_dir() {
            generate_validator_tests_module(test_file, &path)?;
        } else {
            match path.extension() {
                Some(ext) if ext == "yaml" => write_validator_test(test_file, &path)?,
                _ => {}
            };
        }
    }

    end_module(&mut test_file)?;
    Ok(())
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

fn start_validator_module(test_file: &mut File, name: &str) -> Result<(), Error> {
    write!(
        test_file,
        r#"
        #[allow(unused_imports)]
        mod {name} {{
            use reconfix;
            use serde_yaml;
            use serde_json;
        "#,
        name = name
    )?;
    Ok(())
}

fn end_module(test_file: &mut File) -> Result<(), Error> {
    write!(
        test_file,
        r#"
        }}
        "#
    )?;
    Ok(())
}

fn normalize_file_stem(path: &PathBuf) -> Result<String, Error> {
    let result = path
        .file_stem()
        .ok_or(Error {
            message: "cannot convert os string into string".to_string(),
        })?
        .to_string_lossy()
        .replace("-", "_");
    Ok(result.to_string())
}

fn write_validator_test(test_file: &mut File, path: &PathBuf) -> Result<(), Error> {
    let name = format!("{}", normalize_file_stem(&path)?);

    write!(
        test_file,
        include_str!("./tests/validator_test_template"),
        name = name,
        path = path.display()
    )?;
    Ok(())
}
