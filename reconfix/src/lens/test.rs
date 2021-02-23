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
    let (value, save) = lens.apply_x::<_, ()>(1, None).unwrap();

    assert_eq!(value.as_int(), Some(0));
    assert!(save.is_none())
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
    let (value, save) = lens.invert().apply_x::<_, ()>(1, None).unwrap();

    assert_eq!(value.as_int(), Some(2));
    assert!(save.is_none())
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
    let (value, save) = lens.apply_y::<_, ()>(1, None).unwrap();

    assert_eq!(value.as_int(), Some(2));
    assert!(save.is_none())
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
    let (value, save) = lens.invert().apply_y::<_, ()>(1, None).unwrap();

    assert_eq!(value.as_int(), Some(0));
    assert!(save.is_none())
}
