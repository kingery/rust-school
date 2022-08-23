fn main() {
    println!("Running `ownership`...");

    let s1 = String::from("This string has words");
    let s2 = String::from("nowords");

    let w1 = first_word(&s1);
    let w2 = first_word(&s2);

    //s1.clear();
    //s2.clear();

    println!("first word of string 1: {}", w1);
    println!("first word of string 2: {}", w2);
    /*

    let s1 = String::from("Hello!");
    let mut s2 = String::from("I'm mutable :)");

    let len = calculate_length(&s1);

    let m1 = &mut s2;
    let m2 = &mut s2;

    //println!("{}, {}", m1, m2);// using two mutable references causes issue

    change(&mut s2);
    let len2 = calculate_length(&s2);

    println!("String '{}' has length {}", s1, len);
    println!("String '{}' has length {}", s2, len2);
    */
    /*
    let s = String::from("the string in question");

    takes_ownership(s);

    //println!("{}", s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let s1 = String::from("the string");
    let s2 = s1.clone(); 

    let x = 11;
    let y = x;

    let lit1 = "String literal";
    let lit2 = lit1;

    println!("{}", s2);
    println!("{}, {}", x, y);
    println!("{}", lit2);

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);
    */
}

/*
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("....something extra");
}

fn takes_ownership(s: String) {
    println!("This method has taken ownership!");
    println!("{}", s);
}

fn makes_copy(x: u32) {
    println!("This function makes copy!");
    println!("{}", x);
}
*/

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
