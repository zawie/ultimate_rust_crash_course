

pub fn inspect(arg: &String) {
    if arg.ends_with('s') {
        println!("\"{}\" is plural!", arg);
    } else {
        println!("\"{}\" is NOT plural!", arg);
    }
}

pub fn change(arg: &mut String) {
    if !arg.ends_with('s') {
        arg.push_str("s");
    }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with('b') && arg.contains('a') 
}

pub fn add(a: &i32, b: &i32) -> i32{
    (*a) + (*b)
}