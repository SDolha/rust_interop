struct Object {
    flag: bool,
    data: String,
}
extern "Rust" { 
    fn rust_update(object: Object) -> Object;
}

fn update(object: Object) -> Object {
    unsafe { rust_update(object) }
}

fn main() {
    let object = Object { flag: false, data: "Object Name=\"Rust🦀\"".to_string() };
    let object = update(object);
    println!("flag: {}", object.flag);
    println!("{}", object.data);
}
