pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    OneLine,
    Text,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CategoryAttribute {
    NotRoot,
}

struct Category {}

struct Bond {}

struct Bonds {}

struct Article {}

// 一個板的發文規則
struct Board {}

impl Board {
    // 判斷一篇文章能否在本板發表
    fn can_publish(&self, article: &Article) -> bool {
        unimplemented!();
    }
}
