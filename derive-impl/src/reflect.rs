pub use derive::Reflect;

pub struct Field {
    name: &'static str,
    f_mut: &'static str,
    f_visibility: &'static str,
    f_type: &'static str,
}
pub trait Reflect {
    fn name(&self) -> &'static str;
    fn field_names(&self) -> Vec<&'static str>;
}

#[test]
fn reflect_name() {
    #[derive(Reflect)]
    struct SomeStruct;

    let some_struct = SomeStruct;
    assert!(some_struct.name() == "SomeStruct");

    #[derive(Reflect)]
    struct AnotherStruct;
    let another_struct = AnotherStruct;

    assert_ne!(some_struct.name(), another_struct.name())
}

#[test]
fn reflect_fields() {
    #[derive(Reflect)]
    #[allow(unused)]
    struct SomeStruct {
        field_1: bool,
        field_2: u8,
    }

    let some_struct = SomeStruct {
        field_1: true,
        field_2: 1,
    };
    assert!(some_struct.field_names().get(0) == Some(&"field_1"));
    assert!(some_struct.field_names().get(1) == Some(&"field_2"));
    assert_ne!(
        some_struct.field_names().get(0),
        some_struct.field_names().get(1)
    );

    #[derive(Reflect)]
    #[allow(unused)]
    struct AnotherStruct {
        field_a: bool,
        field_b: u8,
    }

    let another_struct = AnotherStruct {
        field_a: false,
        field_b: 0,
    };
    assert!(another_struct.field_names().get(0) == Some(&"field_a"));
    assert!(another_struct.field_names().get(1) == Some(&"field_b"));

    assert_ne!(
        another_struct.field_names().get(0),
        some_struct.field_names().get(0)
    );
    assert_ne!(
        another_struct.field_names().get(1),
        some_struct.field_names().get(1)
    );
}