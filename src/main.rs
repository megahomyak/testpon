// https://soundcloud.com/geekedhub/meth-pipe

#[derive(Debug)]
enum UndNodeKind {
    Text(String),
    Group(Vec<UndNode>),
}
#[derive(Debug)]
struct UndNode {
    idx: usize,
    kind: UndNodeKind,
}
#[derive(Debug)]
struct UndParsingResult {
    unexpected_closers: Vec<usize>,
    unclosed_openers: Vec<usize>,
    root: Vec<UndNode>,
}
macro_rules! und_get_top {
    ($overlays:ident, $root:ident) => {
        match $overlays.last_mut() {
            None => &mut $root,
            Some(top) => &mut top.group,
        }
    };
}
fn parse_und(input: &str) -> UndParsingResult {
    let mut root = Vec::new();
    struct Overlay {
        idx: usize,
        group: Vec<UndNode>,
    }
    let mut overlays: Vec<Overlay> = vec![];

    let mut unclosed_openers = vec![];
    let mut unexpected_closers = vec![];

    let mut nextidx = 0;
    let mut curidx = nextidx;

    let mut textbuf = String::new();
    let mut textidx = nextidx;

    let mut escaped = false;

    loop {
        let c = unsafe { input.get_unchecked(nextidx..) }.chars().next();
        if let Some(c) = c {
            nextidx += c.len_utf8();
            if escaped {
                escaped = false;
                textbuf.push(c);
                continue;
            } else {
                curidx = nextidx;
            }
        }

        match c {
            None | Some('(') | Some(')') => {
                if !textbuf.is_empty() {
                    let top = und_get_top!(overlays, root);
                    top.push(UndNode {
                        idx: textidx,
                        kind: UndNodeKind::Text(textbuf),
                    });
                    textidx = nextidx;
                    textbuf = String::new();
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
                            let new_top = match overlays.last_mut() {
                                None => &mut root,
                                Some(new_top) => &mut new_top.group,
                            };
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
            Some(c) => {
                if c == '\\' {
                    escaped = true;
                }
                textbuf.push(c);
            }
        }
    }

    UndParsingResult {
        unclosed_openers,
        unexpected_closers,
        root,
    }
}

#[derive(Debug)]
enum PonCommandKind {
    Name(Vec<PonWord>),
    Invocation(Vec<UndNode>),
}
#[derive(Debug)]
struct PonCommand {
    idx: usize,
    kind: PonCommandKind,
}
type PonWord = String;
fn und_to_pon(und: Vec<UndNode>) -> Vec<PonCommand> {
    let mut program = Vec::new();
    for und_node in und {
        match und_node.kind {
            UndNodeKind::Group(input) => {
                program.push(PonCommand {
                    idx: und_node.idx,
                    kind: PonCommandKind::Invocation(input),
                });
            }
            UndNodeKind::Text(plain) => {
                let mut words = Vec::new();
                let mut nextidx = 0;
                let mut wordbuf = String::new();
                let mut wordidx = nextidx;
                loop {
                    let c = unsafe { plain.get_unchecked(nextidx..) }.chars().next();
                    let is_special = match c {
                        None => true,
                        Some(c) => {
                            nextidx += c.len_utf8();
                            if c.is_whitespace() && words.is_empty() {
                                wordidx = nextidx;
                            }
                            c.is_whitespace()
                        }
                    };
                    if is_special {
                        if words.is_empty() {
                            words.push(wordbuf);
                            wordbuf = String::new();
                        }
                        if c == None {
                            break;
                        }
                    }
                }
                if !words.is_empty() {
                    program.push(PonCommand {
                        idx: wordidx,
                        kind: PonCommandKind::Name(words),
                    })
                }
            }
        }
    }
    program
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let und_results = parse_und(&contents);
    println!("Und results: {:#?}", und_results);
    let pon = und_to_pon(und_results.root);
    println!("Pon: {:#?}", pon);
}
