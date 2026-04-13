use base::{partial, partial_right};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn format_name(first: String, last: String, suffix: String) -> String {
    format!("{} {}{}", first, last, suffix)
}

#[test]
fn left_partial_single_remaining_arg() {
    let add_10 = partial!(add, [10]);

    assert_eq!(add_10(5), 15);
}

#[test]
fn right_partial_single_remaining_arg() {
    let plus_10 = partial_right!(add, [10]);

    assert_eq!(plus_10(5), 15);
}

#[test]
fn left_partial_multi_remaining_args() {
    let with_first_name = partial!(format_name, [String::from("Ada")], [last, suffix]);

    assert_eq!(
        with_first_name(String::from("Lovelace"), String::from("!")),
        String::from("Ada Lovelace!")
    );
}

#[test]
fn left_partial_multi_initial_args() {
    let with_first_name = partial!(
      format_name, [String::from("Ada"), String::from("Lovelace")], [suffix]);

    assert_eq!(
        with_first_name(String::from("!")),
        String::from("Ada Lovelace!")
    );
}

#[test]
fn right_partial_multi_remaining_args() {
    let with_suffix = partial_right!(format_name, [String::from("!")], [first, last]);

    assert_eq!(
        with_suffix(String::from("Ada"), String::from("Lovelace")),
        String::from("Ada Lovelace!")
    );
}
