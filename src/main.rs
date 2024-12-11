/// If "first" is "\0", the string is empty
struct NonEmptyString {
    first: char,
    rest: String,
}
fn non_empty_string_push(text: &mut NonEmptyString, c: char) {
    if text.first == '\0' {
        text.first = c;
    } else {
        text.rest.push(c);
    }
}

enum UndNode {
    Text(NonEmptyString),
    Group(Vec<UndNode>),
}

type PonProgram = Vec<PonCommand>;
enum PonCommand {
    Name(Vec<PonWord>),
    Invocation(Vec<UndNode>),
}
type PonWord = NonEmptyString;

fn main() {
    println!("Hello, world!");
}
