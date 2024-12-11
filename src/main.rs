struct NonEmptyString {
    first: char,
    rest: String,
}
macro_rules! non_empty_string_push {
    ($string: ident, $c: expr) => {
        match &mut $string {
            None => {
                $string = Some(NonEmptyString {
                    first: $c,
                    rest: String::new(),
                })
            }
            Some(string) => string.rest.push($c),
        }
    };
}

enum UndNodeKind {
    Text(NonEmptyString),
    Group(Vec<UndNode>),
}
struct UndNode {
    idx: usize,
    kind: UndNodeKind,
}
struct UndParsingResult {
    unexpected_closers: Vec<usize>,
    unclosed_openers: Vec<usize>,
    root: Vec<UndNode>,
    escape_at_end_of_input: bool,
}
macro_rules! und_get_top {
    ($overlays: ident, $root: ident) => {
        match $overlays.last_mut() {
            None => &mut $root,
            Some(top) => &mut top.group,
        }
    };
}
pub fn parse_und(input: &str) -> UndParsingResult {
    let mut root = Vec::new();
    struct Overlay {
        idx: usize,
        group: Vec<UndNode>,
    }
    let mut overlays: Vec<Overlay> = vec![];

    let mut unclosed_openers = vec![];
    let mut unexpected_closers = vec![];

    let mut idx = 0;
    let mut curidx = idx;

    let mut sbuf = None;
    let mut sbufidx = idx;

    let mut escaped = false;

    loop {
        let c = unsafe { input.get_unchecked(idx..) }.chars().next();
        if let Some(c) = c {
            curidx = idx;
            idx += c.len_utf8();
            if escaped {
                escaped = false;
                if !(c == '(' || c == ')' || c == '\\') {
                    non_empty_string_push!(sbuf, '\\');
                }
                non_empty_string_push!(sbuf, c);
                continue;
            }
        }

        match c {
            Some(')') | None | Some('(') => {
                if let Some(filled_sbuf) = sbuf {
                    let top = und_get_top!(overlays, root);
                    top.push(UndNode {
                        idx: sbufidx,
                        kind: UndNodeKind::Text(filled_sbuf),
                    });
                    sbufidx = idx;
                    sbuf = None;
                }
                if c == Some(')') || c == None {
                    match overlays.pop() {
                        None => {
                            if c == None {
                                break;
                            } else {
                                unexpected_closers.push(curidx);
                            }
                        }
                        Some(old_top) => {
                            let new_top = und_get_top!(overlays, root);
                            new_top.push(UndNode {
                                idx: old_top.idx,
                                kind: UndNodeKind::Group(old_top.group),
                            });
                            if c == None {
                                unclosed_openers.push(old_top.idx);
                            }
                        }
                    }
                } else {
                    overlays.push(Overlay {
                        idx: curidx,
                        group: Vec::new(),
                    });
                }
            }
            Some('\\') => escaped = true,
            Some(c) => non_empty_string_push!(sbuf, c),
        }
    }

    UndParsingResult {
        unclosed_openers,
        unexpected_closers,
        root,
        escape_at_end_of_input: escaped,
    }
}

struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}
macro_rules! non_empty_vec_push {
    ($vec: ident, $item: expr) => {
        match &mut $vec {
            None => {
                $vec = Some(NonEmptyVec {
                    first: $item,
                    rest: Vec::new(),
                })
            }
            Some(string) => string.rest.push($item),
        }
    };
}
enum PonCommandKind {
    Name(NonEmptyVec<PonWord>),
    Invocation(Vec<UndNode>),
}
struct PonCommand {
    idx: usize,
    kind: PonCommandKind,
}
type PonWord = NonEmptyString;
fn und_to_pon(und: Vec<UndNode>) -> Vec<PonCommand> {
    let mut program = Vec::new();
    for und_node in und {
        program.push(PonCommand {
            idx: und_node.idx,
            kind: match und_node.kind {
                UndNodeKind::Group(nodes) => PonCommandKind::Invocation(nodes),
                UndNodeKind::Text(text) => {
                    let mut words = None;
                    let mut wordbuf = None;
                    let mut textidx = 0;
                    let mut c = Some(text.first);
                    loop {
                        match c {

                        }
                        c = unsafe { text.rest.get_unchecked(textidx..) }.chars().next();
                        if let Some(c) = c {
                            textidx += c.len_utf8();
                        }
                    }
                }
            },
        });
    }
    program
}

fn main() {
    println!("Hello, world!");
}
