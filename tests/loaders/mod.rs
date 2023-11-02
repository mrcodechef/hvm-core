#![allow(dead_code)]

use hvm_lang::term::{parser, DefNames, DefinitionBook, Term};
use hvmc::{run, ast};
use std::fs;

pub fn load_file(file: &str) -> String {
  let path = format!("{}/tests/programs/{}", env!("CARGO_MANIFEST_DIR"), file);
  fs::read_to_string(path).unwrap()
}

// Parses code and generate a Net from hvm-core syntax
pub fn parse_core(code: &str, size: usize) -> (run::Book, run::Net) {
  let book = ast::do_parse_book(code);
  let book = ast::book_to_runtime(&book);
  let mut rnet = run::Net::new(size);
  rnet.boot(ast::name_to_val("main"));
  (book, rnet)
}

// Parses code and generate DefinitionBook from hvm-lang syntax
pub fn parse_lang(code: &str) -> DefinitionBook {
  parser::parse_definition_book(code).unwrap()
}

// Loads file and generate DefinitionBook from hvm-lang syntax
pub fn load_lang(file: &str) -> DefinitionBook {
  let code = load_file(file);
  parse_lang(&code)
}

// For every pair in the map, replaces all matches of a string the other string
pub fn replace_template(mut code: String, map: &[(&str, &str)]) -> String {
  for (from, to) in map {
    code = code.replace(from, to);
  }
  code
}

pub fn hvm_lang_normal(book: DefinitionBook, size: usize) -> (String, run::Net) {
  let (term, defs, info) = hvm_lang::run_book(book, size).unwrap();
  let term = term.to_string(&defs);
  let mut rnet = run::Net::new(size);
  rnet.anni = info.stats.rewrites.anni;
  rnet.comm = info.stats.rewrites.comm;
  rnet.eras = info.stats.rewrites.eras;
  rnet.dref = info.stats.rewrites.dref;
  rnet.oper = info.stats.rewrites.oper;
  ast::net_to_runtime(&mut rnet, &info.net);
  (term, rnet)
}

pub fn show_net(net: &run::Net) -> String {
  ast::show_net(&ast::net_from_runtime(net))
}