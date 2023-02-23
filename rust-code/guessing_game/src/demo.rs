pub fn demo() {
    // let x = 6;

    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("{}", element);
    }

    println!("====================");
    //  创建string类型值
    let mut s = String::from("hello");
    s.push_str(",world");

    println!("{}", s);
    println!("---------------");
    // move:在heap中存储的值不能move，move后变量失效
    let x = 5;
    let y = x;

    println!("{},{}", x, y);

    println!("==============");
    let x = String::from("hello");
    let y = x;

    println!("{}", y); // Error
    println!("============");
    // println!("{},{}", x, y); // Error

    // let tup: (i32, String) = (10, String::from("123"));

    // 所有简单类型都实现了Copy Trait，若出现了复杂类型，则都没有Copy Trait
    let tup1: (i32, i32) = (10, 11);

    let tup2 = tup1;

    println!("{:?},{:?}", tup1, tup2);

    println!("==========");
}
