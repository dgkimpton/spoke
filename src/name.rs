use crate::parser::{SpanSource, TestCase};
use proc_macro2::Span;

pub(crate) struct CompoundName<'a> {
    parts: NameParts<'a>,
}
impl<'a> CompoundName<'a> {
    pub(crate) fn new() -> Self {
        Self {
            parts: NameParts::new(),
        }
    }
    pub(crate) fn followed_by(mut self, name: &'a Name) -> Self {
        self.parts.push(name);
        self
    }
}

pub(crate) struct NameParts<'a>(Vec<&'a Name>);
impl<'a> NameParts<'a> {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
    pub(crate) fn push(&mut self, name: &'a Name) {
        self.0.push(name)
    }
}

#[derive(Clone)]
pub(crate) struct Name {
    location: Span,
    sanitised: String,
}

impl SpanSource for Name {
    fn span(&self) -> Span {
        self.location
    }
}

impl Name {
    pub(crate) fn new(location: &impl SpanSource, source: impl AsRef<str>) -> Self {
        Self {
            location: location.span(),
            sanitised: sanitise(source.as_ref()),
        }
    }

    pub(crate) fn missing(token: &impl SpanSource, id: usize) -> Name {
        Self::new(
            token,
            if id > 1 {
                format!("missing_name_{}", id)
            } else {
                "missing_name".to_string()
            },
        )
    }
}

pub(crate) trait Nameable {
    fn collect_name_parts<'a>(&'a self, compound: CompoundName<'a>) -> CompoundName<'a>;
}

pub(crate) trait Populator {
    fn populate_test(&self, test: TestCase) -> TestCase;
}

impl<'a> CompoundName<'a> {
    pub(crate) fn function_name(self) -> (String, Span) {
        let mut name = self.parts.0.iter().fold(String::new(), |mut acc, b| {
            if !acc.is_empty() {
                acc.push_str("_");
            }
            acc.push_str(&b.sanitised);
            acc
        });

        let location = match self.parts.0.last() {
            Some(n) => n.location,
            None => Span::call_site(),
        };

        if name.starts_with(|c| !unicode_ident::is_xid_start(c)) {
            name.insert(0, 't');
        }

        (name, location)
    }
}

fn sanitise(text: &str) -> String {
    // this is hit or miss - assume most test names contain some special
    // characters and are likely to expand a bit
    let mut builder = WhitespaceSeparatedWords::new(text.len());

    for c in text.chars() {
        if !builder.consume_with_pending(c) {
            builder.consume_char(c)
        }
    }

    builder.build()
}

struct WhitespaceSeparatedWords {
    pending: Option<char>,
    in_white: bool,
    out: String,
}

impl WhitespaceSeparatedWords {
    fn new(size: usize) -> Self {
        Self {
            pending: None,
            in_white: false,
            out: String::with_capacity(size * 2),
        }
    }

    fn start_white(&mut self) {
        if !self.out.is_empty() {
            self.out.push('_');
        }
        self.in_white = true;
    }

    fn push_word(&mut self, word: &str) {
        if !self.in_white {
            self.out.push('_');
        }
        self.out.push_str(word);
        self.start_white();
    }

    fn push_char(&mut self, c: char) {
        if self.in_white {
            self.in_white = false;
        };

        self.out.push(c);
    }

    fn push_space(&mut self) {
        if !self.in_white {
            self.start_white();
        }
    }

    fn push_underscore(&mut self) {
        self.start_white();
    }

    fn push_pending(&mut self, c: char) {
        self.pending = Some(c);
    }

    fn push_pair_word(&mut self, word: &str) -> bool {
        self.push_word(word);
        true
    }
    fn push_single_word(&mut self, word: &str) -> bool {
        self.push_word(word);
        false
    }

    fn consume_with_pending(&mut self, c: char) -> bool {
        if let Some(opening) = self.pending.take() {
            match opening {
                '[' if c == ']' => self.push_pair_word("brackets"),
                '[' => self.push_single_word("open_bracket"),

                '(' if c == ')' => self.push_pair_word("parens"),
                '(' => self.push_single_word("open_paren"),

                '{' if c == '}' => self.push_pair_word("braces"),
                '{' => self.push_single_word("open_brace"),

                '<' if c == '>' => self.push_pair_word("angle_brackets"),
                '<' => self.push_single_word("open_angle_bracket"),

                '\'' if c == '\'' => self.push_pair_word("single_quotes"),
                '\'' => self.push_single_word("single_quote"),

                '\"' if c == '\"' => self.push_pair_word("quotes"),
                '\"' => self.push_single_word("quote"),

                _ => false,
            }
        } else {
            false
        }
    }

    fn consume_char(&mut self, c: char) {
        match c {
            '_' => self.push_underscore(),
            ',' => self.push_word("comma"),
            '&' => self.push_word("ampersand"),
            '.' => self.push_word("dot"),
            '=' => self.push_word("equals"),
            '/' => self.push_word("slash"),
            '*' => self.push_word("star"),
            '+' => self.push_word("plus"),
            '-' => self.push_word("minus"),
            '^' => self.push_word("hat"),
            '%' => self.push_word("percent"),
            '@' => self.push_word("at"),
            '?' => self.push_word("question_mark"),
            '!' => self.push_word("exclamation"),
            '[' => self.push_pending(c),
            ']' => self.push_word("close_bracket"),
            '(' => self.push_pending(c),
            ')' => self.push_word("close_paren"),
            '{' => self.push_pending(c),
            '}' => self.push_word("close_brace"),
            '<' => self.push_pending(c),
            '>' => self.push_word("close_angle_bracket"),
            ':' => self.push_word("colon"),
            ';' => self.push_word("semicolon"),
            '|' => self.push_word("pipe"),
            '#' => self.push_word("hash"),
            '$' => self.push_word("dollars"),
            '`' => self.push_word("backtick"),
            '~' => self.push_word("tilde"),
            '\\' => self.push_word("backslash"),
            '\'' => self.push_pending(c),
            '\"' => self.push_pending(c),
            c if c.is_whitespace() => self.push_space(),
            c if unicode_ident::is_xid_continue(c) => self.push_char(c),
            c if unicode_ident::is_xid_start(c) => self.push_char(c),
            _ => {}
        }
    }

    fn build(&mut self) -> String {
        if self.in_white {
            self.out.pop();
        }
        self.in_white = false;
        std::mem::take(&mut self.out)
    }
}
