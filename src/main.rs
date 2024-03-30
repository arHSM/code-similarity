use std::{env::args, fs::read_to_string};

use detector::{JsHasher, SimilarityParams};

mod detector;

fn main() {
    let args = args().skip(1).take(2).collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("expected 2 arguments");
        return;
    }

    let file_a = read_to_string(&args[0]).expect("file to exist");
    let file_b = read_to_string(&args[1]).expect("file to exist");

    let a = SimilarityParams::new::<_, JsHasher>(file_a);
    let b = SimilarityParams::new::<_, JsHasher>(file_b);

    println!(
        "{}% (text: {}%, ast: {}%)",
        // Operations are symmetric (i.e a.{x_}similarity(b) == b.{x_}similarity(a))
        a.similarity(&b, 0.8) * 100.0,
        a.text_similarity(&b) * 100.0,
        a.ast_similarity(&b) * 100.0,
    );
}
