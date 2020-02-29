fn main() {
  let mut iter = 0..4;
  println!("{:?}", iter); // 0..4
  println!("{:?}" , iter.next()); // Some(0)
  println!("{:?}", iter); // 1..4
  println!("{:?}" , iter.next()); // Some(1)
  println!("{:?}", iter); // 2..4
  println!("{:?}" , iter.next()); // Some(2)
  println!("{:?}", iter); // 3..4
  println!("{:?}" , iter.next()); // Some(3)
  println!("{:?}", iter); // 4..4
  println!("{:?}" , iter.next()); // None
  println!("{:?}", iter); // 4..4
  assert_eq!(iter.next(), None);  // Nothing, Null, "" because there is a match.
}