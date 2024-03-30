use swc_common::{sync::Lrc, FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::Visit;

use super::simhash::Simhash;

pub fn parse<S>(source: S) -> Module
where
    S: Into<String>,
{
    let cm = Lrc::<SourceMap>::default();

    let fm = cm.new_source_file(FileName::Custom("source.js".into()), source.into());
    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    parser.parse_module().expect("failed to parser module")
}

pub trait AstHasher {
    fn visit(source: String) -> u128;
}

pub struct JsHasher {
    hasher: Simhash,
}

impl AstHasher for JsHasher {
    fn visit(source: String) -> u128 {
        let mut this = Self {
            hasher: Simhash::default(),
        };

        let module = parse(source);
        this.visit_module(&module);

        this.hasher.finish()
    }
}

include!("./visit.rs");
