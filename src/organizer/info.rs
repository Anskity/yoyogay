use std::collections::HashMap;

use crate::parser::types::YoyogayType;

pub struct ProjectInfo<'a> {
    pub functions: HashMap<String, YoyogayFunction<'a>>,
}

pub struct YoyogayFunction<'a> {
    pub params: Vec<YoyogayParameter<'a>>,
    pub return_type: YoyogayType<'a>,
}

pub struct YoyogayParameter<'a> {
    pub name: &'a String,
    pub r#type: YoyogayType<'a>,
}
