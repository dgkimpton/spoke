use std::mem::take;

use crate::{
    code_block::CodeBlock, consumer::*, equality::EqualityOperand, generator::*, name::Name,
    rule::Rule, span_source::SpanSource, token::Token, token_is::TokenIs,
};

pub(crate) enum OperandType {
    Eq(EqualityOperand),
}

pub(crate) struct Operand {
    name: Option<Name>,
    code: CodeBlock,
    left: CodeBlock,
    operand_type: Rule<OperandType>,
}

impl Operand {
    pub(crate) fn new(name: Name, code: CodeBlock, left: CodeBlock) -> Self {
        Self {
            name: Some(name),
            code,
            left,
            operand_type: Rule::Uninitialized,
        }
    }
}

impl TokenConsumer for Operand {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match &mut self.operand_type {
            Rule::Uninitialized => match token {
                Token::Token(TokenTree::Ident(otype))
                    if otype.to_string().to_lowercase() == "eq" =>
                {
                    self.operand_type = Rule::found(equality(
                        self.name.take().expect("name to exist"),
                        take(&mut self.code),
                        &otype,
                        take(&mut self.left),
                    ));
                    TokenIs::Consumed
                }

                Token::Token(other) => {
                    self.operand_type = Rule::open_error(
                        other,
                        format!(
                            "expected an assertion operation, one of [eq], in {}",
                            self.name.as_ref().expect("name to exist").full_name()
                        ),
                    );
                    TokenIs::Consumed
                }

                Token::EndOfStream => TokenIs::failed_at_end(format!(
                    "unexpected end of stream in definition of test assertion following $ in{}",
                    self.name.as_ref().expect("name to exist").full_name()
                )),
            },
            Rule::Open(open) => match open.accept_token(token) {
                TokenIs::Rejected(rejected) => {
                    self.operand_type = open.close();
                    TokenIs::Rejected(rejected)
                }
                result => result,
            },
            Rule::Closed(closed) => closed.accept_token(token),
            Rule::OpenError(error) => error.accept_token(token),
            Rule::ClosedError(error) => error.accept_token(token),
        }
    }
}

impl TokenGenerator for Operand {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        self.operand_type.generate_tokens(collector)
    }
}

fn equality(
    name: Name,
    code: CodeBlock,
    anchor_span: &impl SpanSource,
    left: CodeBlock,
) -> OperandType {
    OperandType::Eq(EqualityOperand::new(name, code, anchor_span, left))
}

impl TokenConsumer for OperandType {
    fn accept_token(&mut self, token: Token) -> TokenIs {
        match self {
            OperandType::Eq(op) => op.accept_token(token),
        }
    }
}

impl TokenGenerator for OperandType {
    fn generate_tokens(&mut self, collector: &mut Vec<TokenTree>) {
        match self {
            OperandType::Eq(equality_operand) => equality_operand.generate_tokens(collector),
        }
    }
}
