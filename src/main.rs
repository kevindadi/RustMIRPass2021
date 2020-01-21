#![feature(rustc_private)]

extern crate env_logger;
extern crate getopts;
#[macro_use]
extern crate log;
// extern crate log_settings;
extern crate rustc;
extern crate rustc_metadata;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_codegen_utils;
extern crate rustc_interface;
extern crate syntax;

use rustc_interface::{interface, Queries};
use rustc::hir::def_id::LOCAL_CRATE;
use rustc_driver::Compilation;
use rustc::mir;
use rustc::mir::traversal;

struct HelloCalls {
    data: i32,
}

impl rustc_driver::Callbacks for HelloCalls {
    fn config(&mut self, _config: &mut interface::Config) {
        println!("Hello Config");
    }

    fn after_parsing<'tcx>(&mut self, compiler: &interface::Compiler, queries: &'tcx Queries<'tcx>,) -> Compilation {
        println!("Hello After Parsing");
        compiler.session().abort_if_errors();

        queries.global_ctxt().unwrap().peek_mut().enter(|tcx| {
            let (entry_def_id, _) = tcx.entry_fn(LOCAL_CRATE).expect("no main function found!");
            println!(entry_def_id);
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
    println!("rustc args: {:?}", rustc_args);
    rustc_driver::install_ice_hook();
    let result = rustc_driver::catch_fatal_errors(move || {
        println!("Enter Callback");
        rustc_driver::run_compiler(&rustc_args, &mut HelloCalls {data: 1}, None, None)
    }).and_then(|result| result);
    println!("After Callback: {}", result.is_err() as i32);
    std::process::exit(result.is_err() as i32);
}
