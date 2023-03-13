use std::{collections::HashMap, thread, time::Duration};

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> U
    {
        match self.value.get(&arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    closure_fn(1000);
}

pub fn closure_fn(num: u32) {
    let mut closure = Cacher::new(|num| {
        println!("it's slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if num > 10 {
        if num > 5 {
            println!("some num: {:?}", closure.value(num));
            println!("next num: {:?}", closure.value(num));
        } else {
            if num < 100 {
                println!("1-100");
            } else {
                println!("big num: {}", closure.value(num))
            }
        }
    }
}
