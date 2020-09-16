#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(const_generics)]

pub mod editable;
pub mod input;
pub mod output;
pub mod schema;
pub mod types;

pub extern crate json;

use input::Input;
use output::Output;
use schema::*;
use std::{any::Any, collections::HashMap, error::Error};
use thiserror::Error;
pub use vision_traits_derive::*;

pub type DynErr = Box<dyn Error>;
pub type DynErrResult<T> = Result<T, DynErr>;

pub trait Configurable: Sized + 'static {
    fn schema() -> HashMap<String, SettingType>;
    // TODO: Change this garbage
    fn deserialize(input: &str) -> Result<Self, DeserializationError>;
}

impl Configurable for () {
    fn schema() -> HashMap<String, SettingType> {
        HashMap::new()
    }
    fn deserialize(_: &str) -> Result<Self, DeserializationError> {
        Ok(())
    }
}

pub trait Node: Sized + 'static {
    const NAME: &'static str;

    type S: Configurable;
    type I<'a>: Input<'a>;
    type O: Output;

    fn make(settings: Self::S) -> DynErrResult<Self>;
    fn process(&mut self, input: Self::I<'_>) -> DynErrResult<Self::O>;
}

#[derive(Error, Debug)]
pub enum DeserializationError {
    #[error("field named `{0}` had an invalid type")]
    TypeError(String),
    #[error("field named: `{0}` was not found")]
    MissingField(String),
    #[error("field named: `{0}` had error")]
    FieldDeserializationError(String, Box<dyn Error>),
    // TODO: REMOVE THIS GARBAGE
    #[error("json string is not an object")]
    NotObject,
}

#[derive(Error, Debug)]
pub enum NodeCreationError {
    #[error("input could not be deserialized")]
    DeserializationError(#[from] DeserializationError),
    #[error("node creation failed")]
    CreationError(#[from] DynErr),
}

#[derive(Error, Debug)]
pub enum NodeProcessingError {
    #[error("input could not be deserialized")]
    DeserializationError(#[from] DeserializationError),
    #[error("node execution failed")]
    ExecutionError(#[from] DynErr),
}

pub trait NodeProcessable {
    fn get_schema() -> Function
    where
        Self: Sized;
    fn make(input: &str) -> Result<Box<dyn NodeProcessable>, NodeCreationError>
    where
        Self: Sized;
    fn process(
        &mut self,
        input: &HashMap<String, &dyn Any>,
    ) -> Result<HashMap<String, Box<dyn Any>>, NodeProcessingError>;
}

impl<T: Node> NodeProcessable for T {
    fn get_schema() -> Function {
        Function {
            name: T::NAME.to_owned(),
            settings: T::S::schema(),
            inputs: T::I::<'_>::schema(),
            outputs: T::O::schema(),
        }
    }

    fn make(input: &str) -> Result<Box<dyn NodeProcessable>, NodeCreationError> {
        Ok(Box::new(T::make(T::S::deserialize(&input)?)?))
    }

    fn process<'a>(
        &mut self,
        input: &'a HashMap<String, &dyn Any>,
    ) -> Result<HashMap<String, Box<dyn Any>>, NodeProcessingError> {
        Ok(self
            .process(T::I::<'a>::from_any_map(input)?)?
            .to_any_map())
    }
}
