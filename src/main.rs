use tokenizer::Tokenizer;

mod tokenizer;

fn main() {
    let text = "a = fun(34) + fun(b) / fun(c) ";
    let tokenizer = Tokenizer::new(&text);

    for val in tokenizer {
        print!("{:?} , ", val);
    }
}
