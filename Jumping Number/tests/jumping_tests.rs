use jumping::jumping_number;

#[test]
fn test_jumping1() {
    assert_eq!(jumping_number(1), "Jumping!!");
}

#[test]
fn test_jumping2() {
    assert_eq!(jumping_number(7), "Jumping!!");
}

#[test]
fn test_jumping3() {
    assert_eq!(jumping_number(9), "Jumping!!");
}

#[test]
fn test_jumping4() {
    assert_eq!(jumping_number(23), "Jumping!!");
}

#[test]
fn test_jumping5() {
    assert_eq!(jumping_number(32), "Jumping!!");
}

#[test]
fn test_jumping6() {
    assert_eq!(jumping_number(79), "Not!!");
}

#[test]
fn test_jumping7() {
    assert_eq!(jumping_number(98), "Jumping!!");
}

#[test]
fn test_jumping8() {
    assert_eq!(jumping_number(8987), "Jumping!!");
}

#[test]
fn test_jumping9() {
    assert_eq!(jumping_number(4343456), "Jumping!!");
}

#[test]
fn test_jumping10() {
    assert_eq!(jumping_number(98789876), "Jumping!!");
}