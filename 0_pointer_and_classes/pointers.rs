fn main() {
    let num = 1;
    let ptr_num = &num;

    let my_string1 = String::from("hanifan");
    let my_string2 = &my_string1;

    println!("num: {}, ptr_num: {}, address: {:p}", num, ptr_num, ptr_num);
    println!("{}{}", my_string1, my_string2);
}
