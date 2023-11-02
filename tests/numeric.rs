use hvmc::{
  ast::Book,
  run,
  run::NumericOp,
};
use insta::{assert_debug_snapshot, assert_snapshot};
use loaders::*;

mod loaders;

fn op_net(lnum: u32, op: NumericOp, rnum: u32) -> (run::Book, run::Net) {
  parse_core(&format!("@main = root & <#{lnum} <#{rnum} root>> ~ #{op}"), 16)
}

#[test]
fn test_add() {
  let (book, net) = op_net(10, run::ADD, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#12");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_sub() {
  let (book, net) = op_net(10, run::SUB, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#8");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_mul() {
  let (book, net) = op_net(10, run::MUL, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#20");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_div() {
  let (book, net) = op_net(10, run::DIV, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#5");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_mod() {
  let (book, net) = op_net(10, run::MOD, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#0");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_eq() {
  let (book, net) = op_net(10, run::EQ, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#0");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_ne() {
  let (book, net) = op_net(10, run::NE, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#1");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_lt() {
  let (book, net) = op_net(10, run::LT, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#0");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_gt() {
  let (book, net) = op_net(10, run::GT, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#1");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_and() {
  let (book, net) = op_net(10, run::AND, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#2");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_or() {
  let (book, net) = op_net(10, run::OR, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#10");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_xor() {
  let (book, net) = op_net(10, run::XOR, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#8");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_not() {
  let (book, net) = op_net(0, run::NOT, 256);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#16776959");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_lsh() {
  let (book, net) = op_net(10, run::LSH, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#40");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_rsh() {
  let (book, net) = op_net(10, run::RSH, 2);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#2");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
/// Division by zero always return the value of 0xFFFFFF,
/// that is read as the unsigned integer `16777215`
fn test_div_by_0() {
  let (book, net) = op_net(9, run::DIV, 0);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"#16777215");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
// TODO: we lack a way to check if it's actually doing the chained ops optimization, or if it's doing one op per interaction
fn test_chained_ops() {
  let net = load_lang("chained_ops.hvm");
  let (_, net) = hvm_lang_normal(net, 256);

  assert_snapshot!(show_net(&net), @"#2138224");
  assert_debug_snapshot!(net.rewrites(), @"88");
}
