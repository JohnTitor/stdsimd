use bug::*;

#[test]
fn foo() {
    assert!(ne(V([0.; 4]), V([1.; 4])).0[0] == -1);
}
