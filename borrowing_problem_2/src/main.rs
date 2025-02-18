fn clone_and_modify(s: &String) -> String {
    let mut new_word = s.clone();
    new_word.push_str("World!");
    new_word
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}