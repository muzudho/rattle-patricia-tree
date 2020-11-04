pub mod character_relation;
pub mod string_relation;
pub mod tree_shaped;

pub struct CharacterRelation<T> {
    character: T,
}
pub struct StringRelation<T> {
    string: Vec<T>,
}
pub struct TreeShapedRelation<T> {
    root: Box<StringRelation<T>>,
}
