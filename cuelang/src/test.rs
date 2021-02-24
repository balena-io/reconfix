use crate::{ast::File, Error, Runtime};

#[test]
fn compilation_should_fail_with_the_correct_error_variant() {
    match Runtime::new().compile("", "invalid syntax") {
        Err(Error::Compile(_)) => (),
        Err(err) => panic!("unexpected error: {:?}", err),
        Ok(_) => panic!("unexpected success"),
    }
}

#[test]
fn instance_get_should_return_error_on_unknown_keys() {
    assert!(Runtime::new()
        .compile("", "id: 0")
        .unwrap()
        .get(&["msg"])
        .is_err())
}

#[test]
fn compile_and_retrieve_value() {
    assert_eq!(
        &*Runtime::new()
            .compile(
                "",
                r#"
                msg:   "Hello \(place)!"
                place: string | *"world" // "world" is the default.
                "#,
            )
            .unwrap()
            .get(&["msg"])
            .unwrap()
            .as_string()
            .unwrap(),
        "Hello world!"
    );
}

#[test]
fn compile_ast_and_retrieve_value() {
    assert_eq!(
        &*Runtime::new()
            .compile_ast(
                &File::parse(
                    "",
                    r#"
                    msg:   "Hello \(place)!"
                    place: string | *"world" // "world" is the default.
                    "#,
                )
                .unwrap()
            )
            .unwrap()
            .get(&["msg"])
            .unwrap()
            .as_string()
            .unwrap(),
        "Hello world!"
    );
}
