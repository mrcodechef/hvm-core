#![cfg_attr(feature = "trace", feature(const_type_name))]
#![allow(non_snake_case)]
#![feature(return_position_impl_trait_in_trait)]

pub mod ast;
pub mod jit;
pub mod ops;
pub mod run;
pub mod trace;

pub mod gen;
