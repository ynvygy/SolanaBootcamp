fn main() {
  fizz_buzz()
}

fn fizz_buzz() {
  let mut i = 1;
  let mut times_occured = 0;

  while i < 301 {
      if i%15 == 0 {
          println!("fizzbuzz");
          times_occured +=1
      } else if  i%5 == 0 {
          println!("buzz");
      } else if i%3 == 0 {
          println!("fizz");
      }
      i+=1;
  }
  println!("{}", times_occured)
}