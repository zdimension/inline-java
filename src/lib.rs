#![feature(min_specialization)]

use std::any::Any;
use j4rs::{Instance};
use serde::de::DeserializeOwned;
pub use crate::context::Context;

pub use inline_java_macros::java;

mod context;

pub struct JavaCode {
    code: &'static str
}

impl JavaCode {
    pub fn new(code: &'static str) -> Self {
        Self {
            code
        }
    }
}

pub trait Runner {
    fn run_code(code: JavaCode) -> Self;
}
/*
impl Runner for Instance {
    fn run_code(code: JavaCode) -> Self {
        let ctx = Context::new();
        ctx.run(code)
    }
}
*/

pub struct JavaVal {
    context: Context,
    inst: Instance
}

impl JavaVal {
    pub fn to<T>(self) -> T
    where
        T: DeserializeOwned + Any
    {
        self.context.jvm.to_rust(self.inst).unwrap()
    }
}

pub fn to<T>(val: JavaVal) -> T
where
    T: DeserializeOwned + Any
{
    val.to()
}

impl Runner for () {
    fn run_code(code: JavaCode) -> Self {
        let ctx = Context::new();
        ctx.run(code);
    }
}

impl Runner for Context {
    fn run_code(code: JavaCode) -> Self {
        let ctx = Context::new();
        ctx.run(code);
        ctx
    }
}

impl Runner for JavaCode {
    fn run_code(code: JavaCode) -> Self {
        code
    }
}

impl Runner for JavaVal {
    fn run_code(code: JavaCode) -> Self {
        let ctx = Context::new();
        JavaVal {
            inst: ctx.run(code),
            context: ctx,
        }
    }
}
/*
impl<T> Runner for T
    where T: DeserializeOwned + Any
{
    default fn run_code(code: JavaCode) -> Self {
        let ctx = Context::new();
        let res = ctx.run(code);
        ctx.jvm.to_rust(res).unwrap()
    }
}*/