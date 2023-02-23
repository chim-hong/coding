use std::{io, cmp::Ordering};
use rand::Rng;


pub fn guess_fun(){
  println!("猜数！");

  let secret_number = rand::thread_rng().gen_range(1..101);
  loop {
      println!("猜测一个数：");
      // let用于指定常量，mut表示该常量可以编辑
      let mut guess = String::new();
      io::stdin().read_line(&mut guess).expect("无法读取行！");
      println!("你猜测的数是：{}", guess);
      // showdow
      let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
      };
      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too Small!"),
          Ordering::Equal => {
              println!("success!");
              break;
          }
          Ordering::Greater => println!("Too larger!"),
      }
  }
}
