#![allow(dead_code)]

use hvml::term::{parser, Book as DefinitionBook, DefId, DefNames, term_to_net::Labels};
use hvmc::{ast::*, run::{self, Area}};
use std::{collections::HashMap, fs};

pub fn load_file(file: &str) -> String {
  let path = format!("{}/tests/programs/{}", env!("CARGO_MANIFEST_DIR"), file);
  fs::read_to_string(path).unwrap()
}

// Parses code and generate Book from hvm-core syntax
pub fn parse_core(code: &str) -> Book {
  do_parse_book(code)
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

// For every pair in the map, replaces all matches of a string with the other string
pub fn replace_template(mut code: String, map: &[(&str, &str)]) -> String {
  for (from, to) in map {
    code = code.replace(from, to);
  }
  code
}

pub fn hvm_lang_readback(net: &Net, book: &DefinitionBook) -> (String, bool) {
  let net = hvml::net::hvmc_to_net::hvmc_to_net(net);
  let (res_term, valid_readback) = hvml::term::net_to_term::net_to_term(&net, book, &Labels::default(), true);

  (format!("{:?}", res_term), valid_readback.is_empty())
}

pub fn hvm_lang_normal(book: &mut DefinitionBook, size: usize) -> (hvmc::run::Rewrites, Net) {
  let compiled = hvml::compile_book(book, hvml::OptimizationLevel::Light).unwrap();
  let (root, res_lnet) = normal(compiled.core_book, size);
  (root, res_lnet)
}

#[allow(unused_variables)]
pub fn normal(book: Book, size: usize) -> (hvmc::run::Rewrites, Net) {
  fn normal_cpu<'area>(host: &Host, area: &'area Area) -> run::Net<'area> {
    let mut rnet = run::Net::new(area);
    rnet.boot(host.defs.get(DefNames::ENTRY_POINT).unwrap());
    rnet.normal();
    rnet
  }

  #[cfg(feature = "cuda")]
  fn normal_gpu(book: run::Book) -> run::Net {
    let (_, host_net) = hvmc::cuda::host::run_on_gpu(&book, "main").unwrap();
    host_net.to_runtime_net()
  }

  let area = run::Net::init_heap(size);
  let host = Host::new(&book);

  let rnet = {
    #[cfg(not(feature = "cuda"))]
    {
      normal_cpu(&host, &area)
    }
    #[cfg(feature = "cuda")]
    {
      normal_gpu(book)
    }
  };

  let net = host.readback(&rnet);
  (rnet.rewrites(), net)
}
