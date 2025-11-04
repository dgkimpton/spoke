use proc_macro2::Span;
use std::{cell::RefCell, rc::Rc};

use crate::{
    error::{CompileError, CompileResult},
    span_source::SpanSource,
};

#[derive(Clone)]
pub(crate) struct NameFactory(Rc<RefCell<Factory>>);

impl NameFactory {
    pub(crate) fn new() -> Self {
        Self(Rc::new(RefCell::new(Factory::new())))
    }

    pub(crate) fn make_factory(&self, title: String) -> Self {
        Self(Rc::new(RefCell::new(Factory::new_with_parent(
            self.clone(),
            Some(title),
        ))))
    }

    pub(crate) fn make_name(&self, sp: &impl SpanSource, text: String) -> Name {
        self.0.borrow_mut().make_name(sp, text, self.clone())
    }

    pub(crate) fn qualified_name(&self, sp: &impl SpanSource) -> CompileResult<String> {
        let imp = self.0.borrow();
        self.assemble_name(sp, imp.title.as_ref().unwrap(), " ⟶ ")
    }

    fn assemble_name(&self, sp: &impl SpanSource, text: &str, sep: &str) -> CompileResult<String> {
        if text.is_empty() {
            return CompileError::err(sp, "test segment names cannot be empty");
        }

        let imp = self.0.borrow();

        if !imp.has_parent() {
            if let Some(ref title) = imp.title {
                if title.is_empty() {
                    return CompileError::err(sp, "test segment names cannot be empty");
                } else {
                    return Ok(format!("{}{}{}", title, sep, text));
                }
            }

            return Ok(text.into());
        }

        Ok(format!(
            "{}{}{}",
            imp.parent
                .as_ref()
                .unwrap()
                .assemble_name(sp, imp.title.as_ref().unwrap(), sep)?,
            sep,
            text
        ))
    }

    fn max_index(&self) -> usize {
        self.0.as_ref().borrow().provided_name_count
    }

    pub(crate) fn has_parent(&self) -> bool {
        self.0.as_ref().borrow().has_parent()
    }
}

struct Factory {
    parent: Option<NameFactory>,
    title: Option<String>,
    provided_name_count: usize,
}

impl Factory {
    fn new() -> Self {
        Self {
            parent: None,
            title: None,
            provided_name_count: 0,
        }
    }

    pub(crate) fn new_with_parent(parent: NameFactory, title: Option<String>) -> Self {
        Self {
            parent: Some(parent),
            title,
            provided_name_count: 0,
        }
    }

    pub(crate) fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub(crate) fn make_name(
        &mut self,
        sp: &impl SpanSource,
        text: String,
        factory: NameFactory,
    ) -> Name {
        Name::new(sp, factory, text, self.next_index())
    }

    fn next_index(&mut self) -> usize {
        self.provided_name_count += 1;
        self.provided_name_count
    }
}

#[derive(Clone)]
pub(crate) struct Name {
    factory: NameFactory,
    span: Span,
    text: String,
    index: usize,
}

impl Name {
    pub(crate) fn new(
        sp: &impl SpanSource,
        factory: NameFactory,
        text: String,
        index: usize,
    ) -> Self {
        Self {
            factory,
            span: sp.span(),
            text,
            index,
        }
    }

    pub(crate) fn span(&self) -> &proc_macro2::Span {
        &self.span
    }

    pub(crate) fn make_factory(&self) -> NameFactory {
        self.factory.make_factory(self.text.clone())
    }

    pub(crate) fn function_name(&self) -> CompileResult<String> {
        Ok(sanitise(
            self.factory
                .assemble_name(self.span(), &self.text, " ")?
                .as_str(),
            self.index,
            self.factory.max_index(),
        ))
    }

    pub(crate) fn full_name(&self) -> CompileResult<String> {
        self.factory.assemble_name(self.span(), &self.text, " ⟶ ")
    }
}

const MAX_LENGTH: usize = 900;

fn sanitise(text: &str, index: usize, max_index: usize) -> String {
    // this is hit or miss - assume most test names contain some special
    // characters and are likely to expand a bit
    let mut builder = WhitespaceSeparatedWords::new(text.len());

    for c in text.chars() {
        if !builder.consume_with_pending(c) {
            builder.consume_char(c)
        }
        if builder.out.len() > MAX_LENGTH {
            break;
        }
    }

    builder.build(index, max_index)
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

        if self.out.is_empty() && !unicode_ident::is_xid_start(c) {
            self.out.push('t');
        }
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

    fn build(&mut self, index: usize, max_index: usize) -> String {
        if self.in_white {
            self.out.pop();
        }
        self.in_white = false;
        if self.out.len() > MAX_LENGTH {
            self.out.truncate(MAX_LENGTH);
            self.out.push_str(format_index(index, max_index).as_str());
        }
        std::mem::take(&mut self.out)
    }
}

fn format_index(index: usize, max_index: usize) -> String {
    let digits = (max_index as f64 + 0.1).log10().ceil() as usize;
    format!("_{:0width$}", index, width = digits)
}

#[cfg(test)]
mod tests {
    use super::format_index;

    #[test]
    fn format_index_works_correctly() {
        assert_eq!("_0", format_index(0, 1));
        assert_eq!("_1", format_index(1, 1));
        assert_eq!("_00", format_index(0, 10));
        assert_eq!("_07", format_index(7, 12));
        assert_eq!("_17", format_index(17, 17));
        assert_eq!("_1889", format_index(1889, 2000));
    }
}
