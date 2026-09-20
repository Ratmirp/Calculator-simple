use calc::solve;

#[test]
fn all_cases() {
    let cases: &[(&str, f64)] = &[
        ("6 + 2", 8.0),
        ("5 + 3", 8.0),
        ("2 + 3 * 4", 14.0),
        ("(2 + 3) * 4", 20.0),
        ("-5 - 10", -15.0), // I am lazy
    ];

    for (input, expected) in cases {
        assert_eq!(solve(input.to_string()), Ok(*expected));
    }
}
