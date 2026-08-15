// A lifetime annotation (`'a`) describes how borrowed references relate to
// each other. It does not make either value live longer.
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

struct Excerpt<'a> {
    text: &'a str,
}

fn main() {
    let first = String::from("Rust lifetimes");

    {
        let second = String::from("borrow checker");
        let result = longest(first.as_str(), second.as_str());

        println!("The longer phrase is: {result}");

        // `result` is used only while both `first` and `second` are alive.
        // The `'a` in `longest` tells Rust that the returned reference cannot
        // outlive the shorter-lived input reference.
    }

    let sentence = String::from("Lifetimes describe relationships. They do not extend ownership.");
    let first_sentence = sentence
        .split('.')
        .next()
        .expect("the example sentence is not empty");
    let excerpt = Excerpt {
        text: first_sentence,
    };

    // `Excerpt<'a>` cannot outlive `sentence` because it borrows from it.
    println!("Excerpt: {}", excerpt.text);
    println!("{}", sentence);
}
