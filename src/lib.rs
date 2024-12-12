pub mod ast;
pub mod organizer;
pub mod text_data;
pub mod tokenizer;
pub mod parser;
pub mod compiler;

pub mod tests;

pub trait Boxxable {
    fn to_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<T: Sized> Boxxable for T {}
