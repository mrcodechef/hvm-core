mod loaders;

#[cfg(not(feature = "cuda"))] // FIXME: Cuda does not support native numbers
mod numeric_tests {
  use crate::loaders::*;
  use hvmc::{
    ast::{show_net, Book},
    run::{self, Tag}, ops::Op,
  };
  use insta::{assert_debug_snapshot, assert_snapshot};

  fn op_net(lnum: u32, op: Op, rnum: u32) -> Book {
    println!("Code: {:?}", &format!("@main = root & #{lnum} ~ <{op} #{rnum} root>"));
    parse_core(&format!("@main = root & #{lnum} ~ <{op} #{rnum} root>"))
  }

  #[test]
  fn test_add() {
    let net = op_net(10, Op::Add, 2);
    let (rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#12");
    assert_debug_snapshot!(rwts.total(), @"2");
  }

  #[test]
  fn test_sub() {
    let net = op_net(10, Op::Sub, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#8");
  }

  #[test]
  fn test_mul() {
    let net = op_net(10, Op::Mul, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#20");
  }

  #[test]
  fn test_div() {
    let net = op_net(10, Op::Div, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#5");
  }

  #[test]
  fn test_mod() {
    let net = op_net(10, Op::Mod, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#0");
  }

  #[test]
  fn test_eq() {
    let net = op_net(10, Op::Eq, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#0");
  }

  #[test]
  fn test_ne() {
    let net = op_net(10, Op::Ne, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#1");
  }

  #[test]
  fn test_lt() {
    let net = op_net(10, Op::Lt, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#0");
  }

  #[test]
  fn test_gt() {
    let net = op_net(10, Op::Gt, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#1");
  }

  #[test]
  fn test_and() {
    let net = op_net(10, Op::And, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#2");
  }

  #[test]
  fn test_or() {
    let net = op_net(10, Op::Or, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#10");
  }

  #[test]
  fn test_xor() {
    let net = op_net(10, Op::Xor, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#8");
  }

  #[test]
  fn test_not() {
    let net = op_net(0, Op::Not, 256);
    let (rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#16776959");
    assert_debug_snapshot!(rwts.total(), @"2");
  }

  #[test]
  fn test_lsh() {
    let net = op_net(10, Op::Lsh, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#40");
  }

  #[test]
  fn test_rsh() {
    let net = op_net(10, Op::Rsh, 2);
    let (_rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#2");
  }

  #[test]
  /// Division by zero always return the value of 0xFFFFFF,
  /// that is read as the unsigned integer `16777215`
  fn test_div_by_0() {
    let net = op_net(9, Op::Div, 0);
    let (rwts, net) = normal(net, 16);
    assert_snapshot!(show_net(&net), @"#16777215");
    assert_debug_snapshot!(rwts.total(), @"5");
  }

  #[test]
  // TODO: we lack a way to check if it's actually doing the chained ops optimization, or if it's doing one op per interaction
  fn test_chained_ops() {
    let mut net = load_lang("chained_ops.hvm");
    let (rwts, net) = hvm_lang_normal(&mut net, 256);

    assert_snapshot!(show_net(&net), @"#2138224");
    assert_debug_snapshot!(rwts.total(), @"36");
  }
}
