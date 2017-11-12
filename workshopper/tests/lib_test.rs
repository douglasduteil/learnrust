extern crate workshopper;

#[test]
fn it_should_extract_exercise_titles () {
  // given
  use workshopper::get_exercice_titles;

  let files = vec![
    String::from("exo/foo/oof.x"),
    String::from("exo/bar/rab.x"),
    String::from("exo/qux/xuq.x"),
    String::from("exo/qux/xoq.x")
  ];

  // when
  let actual = get_exercice_titles(&files);

  // then
  assert_eq!(actual, vec!["foo", "bar", "qux"]);
}
