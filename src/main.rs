#![feature(rustc_private)]

extern crate env_logger;
// extern crate getopts;
#[macro_use]
extern crate log;
// extern crate log_settings;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_hir;

use rustc_interface::{interface, Queries};
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_driver::Compilation;
use rustc_middle::mir;
use rustc_middle::mir::traversal;

struct HelloCalls {
    data: i32,
}

impl HelloCalls {
    fn new() -> Self{
        Self {
            data: 1i32
        }
    }
}

impl rustc_driver::Callbacks for HelloCalls {
    fn config(&mut self, _config: &mut interface::Config) {
        println!("Hello Config");
    }

    fn after_parsing<'tcx>(&mut self, compiler: &interface::Compiler, queries: &'tcx Queries<'tcx>,) -> Compilation {
        println!("Hello After Parsing");
        compiler.session().abort_if_errors();

        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            let (entry_def_id, _) = tcx.entry_fn(()).expect("no main function found!");
            println!("{:?}", entry_def_id);
        });
        Compilation::Stop
    }

    fn after_analysis<'tcx>(&mut self, compiler: &interface::Compiler, queries: &'tcx Queries<'tcx>,) -> Compilation {
        println!("Hello After Analsysis");
        compiler.session().abort_if_errors();
        
        Compilation::Continue
    }
}

fn main() {
    rustc_driver::init_rustc_env_logger();
    let mut rustc_args = vec![];
    for arg in std::env::args() {
        rustc_args.push(arg);
    }
    let sysroot: String = "--sysroot".into();
    rustc_args.push(sysroot);
    rustc_args.push(find_sysroot());
    println!("rustc args: {:?}", rustc_args);
    rustc_driver::install_ice_hook();
    let result = rustc_driver::catch_fatal_errors(move || {
        println!("Enter Callback");
        let mut helloCalls = HelloCalls::new();
        let compiler = rustc_driver::RunCompiler::new(&rustc_args, &mut helloCalls);
        compiler.run()
    }).and_then(|result| result);
    println!("After Callback: {}", result.is_err() as i32);
    std::process::exit(result.is_err() as i32);
}

fn find_sysroot() -> String {
    let home = option_env!("RUSTUP_HOME");
    let toolchain = option_env!("RUSTUP_TOOLCHAIN");
    match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{}/toolchains/{}", home, toolchain),
        _ => option_env!("RUST_SYSROOT")
            .expect(
                "Could not find sysroot. Specify the RUST_SYSROOT environment variable, \
                 or use rustup to set the compiler to use for LOCKBUD",
            )
            .to_owned(),
    }
}
