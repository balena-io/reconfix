use super::Lens;
use crate::Error;

#[test]
fn create_simple_lens() {
    Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();
}

#[test]
fn creating_lens_with_package_should_fail() {
    let lens = Lens::new(
        r#"
        package test

        X: Y + 1
        Y: X - 1
        "#,
    );

    match lens {
        Err(Error::InvalidLens(_, err)) => {
            assert_eq!(err.to_string(), "found package declaration")
        }
        _ => panic!(),
    }
}

#[test]
fn creating_lens_without_x_should_fail() {
    let lens = Lens::new(
        r#"
        Y: 10
        "#,
    );

    match lens {
        Err(Error::InvalidLens(_, err)) => {
            assert_eq!(err.to_string(), "missing field: X")
        }
        _ => panic!(),
    }
}

#[test]
fn creating_lens_without_y_should_fail() {
    let lens = Lens::new(
        r#"
        X: 10
        "#,
    );

    match lens {
        Err(Error::InvalidLens(_, err)) => {
            assert_eq!(err.to_string(), "missing field: Y")
        }
        _ => panic!(),
    }
}

#[test]
fn apply_x() {
    let lens = Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();

    assert_eq!(lens.apply_x(1).unwrap().as_int(), Some(0));
}

#[test]
fn apply_x_on_inverted_lens() {
    let mut lens = Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();

    assert_eq!(lens.invert().apply_x(1).unwrap().as_int(), Some(2));
}

#[test]
fn apply_y() {
    let lens = Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();

    assert_eq!(lens.apply_y(1).unwrap().as_int(), Some(2));
}

#[test]
fn apply_y_on_inverted_lens() {
    let mut lens = Lens::new(
        r#"
        X: Y + 1
        Y: X - 1
        "#,
    )
    .unwrap();

    assert_eq!(lens.invert().apply_y(1).unwrap().as_int(), Some(0));
}
