pub fn hello() -> String {

    let hello = String::from("Hello Rust");
    hello

}

pub fn greet(name: &str) -> String {
    let name: &str = name;
    let greet = format!("Hello {}", name);
    greet


}

pub fn append(mut s: String) -> String {
    s.push_str("!"); 
    s
}
