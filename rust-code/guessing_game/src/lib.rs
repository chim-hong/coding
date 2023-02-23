pub mod lib_mod {
    pub mod test{
      pub fn lib_fn() {
        println!("lib function")
    }
    }
}

pub fn some_fn() {
    lib_mod::test::lib_fn();
}

