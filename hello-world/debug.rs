#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
  println!("{:?} months in a year", 12);
  println!("{1:?} {0:?} is the {actor:?} name", "slater", "Christian", actor="actor's" );

  println!("Now {0:?} will print!", Structure(3));

  println!("Now {:?} will print!", Deep(Structure(7)));

}