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
            curidx = nextidx;
            nextidx += c.len_utf8();
        }
        if escaped {
            escaped = false;
            if !matches!(c, Some('(' | ')')) {
                textbuf.push('\\');
            }
            if let Some(c) = c {
                textbuf.push(c);
            }
            continue;
        }

        match c {
            None | Some('(' | ')') => {
                if !textbuf.is_empty() {
                    let top = und_get_top!(overlays, root);
                    top.push(UndNode {
                        idx: textidx,
                        kind: UndNodeKind::Text(textbuf),
                    });
                    textbuf = String::new();
                }
                textidx = nextidx;
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
            Some(c) => textbuf.push(c),
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
            UndNodeKind::Group(invocation) => {
                program.push(PonCommand {
                    idx: und_node.idx,
                    kind: PonCommandKind::Invocation(invocation),
                });
            }
            UndNodeKind::Text(text) => {
                let mut words = Vec::new();
                let mut nextidx = 0;
                let mut wordbuf = String::new();
                let mut nameidx = nextidx;
                let mut escaped = false;
                loop {
                    let c = unsafe { text.get_unchecked(nextidx..) }.chars().next();
                    if let Some(c) = c {
                        nextidx += c.len_utf8();
                    }
                    if escaped {
                        escaped = false;
                        if match c {
                            None => true,
                            Some(c) => !(c == '\\' || c.is_whitespace()),
                        } {
                            wordbuf.push('\\');
                        }
                        if let Some(c) = c {
                            wordbuf.push(c);
                        }
                        continue;
                    };
                    if match c {
                        None => true,
                        Some(c) => {
                            if c.is_whitespace() && words.is_empty() {
                                nameidx = nextidx;
                            }
                            let is_boundary = c == ')' || c == '(' || c.is_whitespace();
                            if !is_boundary {
                                if c == '\\' {
                                    escaped = true;
                                } else {
                                    wordbuf.push(c);
                                }
                            }
                            is_boundary
                        }
                    } {
                        if !wordbuf.is_empty() {
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
                        idx: nameidx + und_node.idx,
                        kind: PonCommandKind::Name(words),
                    });
                }
            }
        }
    }
    program
}

fn main() {
    let contents = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("Contents: {:#?}", contents);
    let und_results = parse_und(&contents);
    println!("Und: {:#?}", und_results);
    let pon = und_to_pon(und_results.root);
    println!("Pon: {:#?}", pon);
    if !(und_results.unclosed_openers.is_empty() && und_results.unexpected_closers.is_empty()) {
        if !und_results.unclosed_openers.is_empty() {
            println!("Unclosed openers: {:?}", und_results.unclosed_openers);
        }
        if !und_results.unexpected_closers.is_empty() {
            println!("Unexpected closers: {:?}", und_results.unexpected_closers);
        }
        std::process::exit(1);
    }
    for command in pon {
        match command.kind {
            PonCommandKind::Name(name) => todo!(),
            PonCommandKind::Invocation(invocation) => todo!(),
        }
    }
}
