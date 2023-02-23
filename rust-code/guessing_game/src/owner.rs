pub fn main() {
    let y = String::from("hello,world");
    let y = take_ownership(y);
    let x = 32;
    makes_copy(32);

    let len = calc_length(&y);

    println!("{}", x);
    println!("{}", y);

    println!("==========");
    println!("{},{}", y, len);
    println!("==========");

    let result = first_word(&String::from("hello world qbc"));
    println!("result:{}", result);
}

fn take_ownership(mut some_string: String) -> String {
    println!("{}", some_string);
    some_string = String::from("123");
    some_string
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn calc_length(str: &String) -> usize {
    str.len()
}

fn first_word(str: &String) -> String {
    let str_split = str.split_once(' ');
    let result = match str_split {
        Option::Some((x,y)) =>{
            println!("{}",y);
            x
        },
        Option::None => str,
    };

    // 字符串切片是以字符为单位
    let sub_str = "你好中国";

    let sub_str_slice = &sub_str[0..3];

    println!("==========");

    println!("{}",sub_str_slice);
    println!("==========");

    println!("{:?}", result);
    String::from(result)


}
