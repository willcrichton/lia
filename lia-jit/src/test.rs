

#[test]
fn compile_test() {
    use std::path::PathBuf;
    use std::rc::Rc;

    use rustc;
    use rustc::session::build_session;
    use rustc::session::config::{self, basic_options, Input};
    use rustc::dep_graph::DepGraph;
    use rustc_llvm;
    use rustc_lint;
    use rustc_driver::{RustcDefaultCalls, CompilerCalls, diagnostics_registry, handle_options};
    use rustc_driver::target_features;
    use rustc_driver::driver::compile_input;
    use rustc_driver::Compilation;
    use rustc_metadata::cstore::CStore;

    use syntax::parse::token;
    use syntax::diagnostics::registry::Registry;

    let sysroot = PathBuf::from("/Users/will/.multirust/toolchains/nightly-x86_64-apple-darwin");

    let input = Input::Str {
        name: "<jit>".to_string(),
        input: r#"
#[no_mangle]
pub fn test_add(a: i32, b: i32) -> i32 { a + b }
"#.to_string()
    };

    let matches = handle_options(&["rustc".into(), "foo.rs".into(), "--emit".into(), "llvm-ir".into()]).unwrap();
    let mut callbacks = RustcDefaultCalls;

    let mut sopts = basic_options();
    sopts.maybe_sysroot = Some(sysroot);
    sopts.optimize = config::OptLevel::No;
    sopts.crate_types = vec![config::CrateTypeDylib];

    let descriptions = diagnostics_registry();

    callbacks
        .early_callback(&matches, &sopts, &descriptions, sopts.error_format);

    let dep_graph = DepGraph::new(sopts.build_dep_graph());
    let cstore = Rc::new(CStore::new(&dep_graph, token::get_ident_interner()));
    let sess = build_session(
        sopts, &dep_graph, None, Registry::new(&rustc::DIAGNOSTICS), cstore.clone());
    rustc_lint::register_builtins(&mut sess.lint_store.borrow_mut(), Some(&sess));
    let mut cfg = config::build_configuration(&sess);
    target_features::add_configuration(&mut cfg, &sess);

    callbacks
        .late_callback(&matches, &sess, &input, &None, &None);

    let mut cc = callbacks.build_controller(&sess, &matches);
    cc.after_llvm.stop = Compilation::Stop;
    cc.after_llvm.run_callback_on_error = true;
    cc.after_llvm.callback = Box::new(|state| {
        state.session.abort_if_errors();
        let trans = state.trans.unwrap();
        assert_eq!(trans.modules.len(), 1);
        let rs_llmod = trans.modules[0].llmod;
        //unsafe { rustc_llvm::LLVMDumpModule(rs_llmod) };
    });

    compile_input(&sess, &cstore, cfg, &input, &None, &None, None, &cc).unwrap();
}
