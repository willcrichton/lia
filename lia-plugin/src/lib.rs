#![feature(rustc_private, plugin_registrar, quote, box_syntax)]
//#![allow(unused_imports, unused_variables, dead_code)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;
extern crate lia;

mod plugin;

use rustc_plugin::Registry;
use syntax::parse::token::intern;
use syntax::ext::base::SyntaxExtension;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("lia", plugin::expand_lia);
    reg.register_macro("_borrow_type", plugin::expand_borrow_type);
    reg.register_syntax_extension(
        intern("lia_impl_glue"),
        SyntaxExtension::MultiModifier(box plugin::impl_glue));
}
