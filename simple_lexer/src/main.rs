fn main() {
    list_char("int age;");
}

fn list_char(s: &str) {
    for char in s.chars() {
        println!("{}", char);
    }
}
