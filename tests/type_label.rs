use type_label::{Label, impl_label};

#[derive(Label, Debug)]
#[label = "foo label"]
pub struct Foo {}

#[derive(Debug)]
pub struct Bar {}

impl Label for Bar {
    const LABEL: &'static str = "bar label";
}

#[derive(Debug)]
pub struct Baz {}

impl_label!(Baz, "baz label");


fn assert_label<T: Label>(_: T, expected: &'static str) {
    let actual = <T as Label>::LABEL;

    if expected != actual {
        panic!(
            "Your result of {} should match the expected result {}",
            actual, expected
        )
    }
}

#[test]
fn derive_label() {
    assert_label(Foo {}, "foo label");
}

#[test]
fn manual_label() {
    assert_label(Bar {}, "bar label");
}

#[test]
fn macro_rules_label() {
    assert_label(Baz {}, "baz label");
}
