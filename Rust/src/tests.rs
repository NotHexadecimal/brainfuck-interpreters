use super::*;

#[test]
fn echo() {
    let runner = ProgramBuilder::new(",.", Some("a"));
    let res = runner.run();

    assert_eq!(res, "a")
}

#[test]
fn interpreter() {
    const EXPECT: &str = "foo";
    const PROG: &str = "--[----->+<]>.+++++++++..";

    let runner = ProgramBuilder::new(PROG, None);
    let res = runner.run();

    assert_eq!(res, EXPECT);
}

#[test]
fn seek() {
    const EXPECT: &str = "foo";
    const SEEK: &str = ">+>+>+>+>+ [<]>[>] --[----->+<]>.+++++++++..";

    let runner = ProgramBuilder::new(SEEK, None);
    let res = runner.run();
    assert_eq!(res, EXPECT);
}

#[test]
fn clear() {
    const EXPECT: &str = "foo";
    const CLEAR: &str = "+[+]-[-]--[----->+<]>.+++++++++..";

    let runner = ProgramBuilder::new(CLEAR, None);
    let res = runner.run();
    assert_eq!(res, EXPECT);
}
