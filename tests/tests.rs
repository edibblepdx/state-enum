use state_enum::state_enum;

#[rustfmt::skip]
#[state_enum]
#[derive(Debug)]
enum State { A, B, C }

#[test]
fn test() {
    let s = State::default();

    assert_eq!(s, State::A);
    assert_eq!(s.next(), State::B);
    assert_eq!(s.next().next(), State::C);
    assert_eq!(s.next().next().next(), State::A);

    assert_eq!(s, State::A);
    assert_eq!(s.prev(), State::C);
    assert_eq!(s.prev().prev(), State::B);
    assert_eq!(s.prev().prev().prev(), State::A);
}
