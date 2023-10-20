#![allow(dead_code)]
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParsingError {
    message: String,
    error_type: ErrorType,
    idx: usize,
}

impl ParsingError {
    pub fn new(message: String, error_type: ErrorType, idx: usize) -> Self { 
        ParsingError { message, error_type, idx} 
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParsingError {}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    union,
    range,
    unexpected,
    group,
}

