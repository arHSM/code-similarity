use std::mem::size_of_val;

use crate::detector::simhash::Simhash;

use super::ast::AstHasher;

#[derive(Debug)]
pub struct SimilarityParams {
    text: u128,
    ast: u128,
}

impl SimilarityParams {
    pub fn new<S, A>(source: S) -> Self
    where
        S: Into<String>,
        A: AstHasher,
    {
        let source = source.into();
        let mut hasher = Simhash::default();

        for word in source.split_whitespace() {
            hasher.write(word.as_bytes())
        }
        let text = hasher.finish();

        let ast = A::visit(source);

        Self { text, ast }
    }

    pub fn similarity(&self, with: &Self, ast_weight: f64) -> f64 {
        let s_t = self.text_similarity(with);
        let s_a = self.ast_similarity(with);

        let w_a = ast_weight.clamp(0.0, 1.0);
        let w_t = 1.0 - w_a;

        (s_t * w_t) + (s_a * w_a)
    }

    pub fn text_similarity(&self, with: &Self) -> f64 {
        Self::hamming_similarity(self.text, with.text)
    }

    pub fn ast_similarity(&self, with: &Self) -> f64 {
        Self::hamming_similarity(self.ast, with.ast)
    }

    fn hamming_similarity(of: u128, with: u128) -> f64 {
        let hamming = (of ^ with).count_ones() as f64;

        1.0 - (hamming / (size_of_val(&of) as f64))
    }
}
