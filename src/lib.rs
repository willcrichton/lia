#![feature(plugin_registrar, rustc_private, quote, box_patterns)]
#![allow(unused_imports, unused_variables, dead_code)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

mod token;
mod ast;
mod lia;
mod grammar;
mod codegen;
mod plugin;

use rustc_plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lia", plugin::expand_lia);
}
