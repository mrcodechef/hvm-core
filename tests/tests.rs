use insta::{assert_debug_snapshot, assert_snapshot};
use loaders::*;

mod loaders;

#[test]
fn test_era_era() {
  let (book, mut net) = parse_core("@main = * & * ~ *", 16);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"*");
  assert_debug_snapshot!(net.rewrites(), @"2");
}

#[test]
fn test_commutation() {
  let (book, mut net) = parse_core("@main = root & (x x) ~ [* root]", 16);
  net.normal(&book);
  assert_snapshot!(show_net(&net), @"(b b)");
  assert_debug_snapshot!(net.rewrites(), @"5");
}

#[test]
fn test_bool_and() {
  let (book, mut net) = parse_core(
    "
    @true = (b (* b))
    @fals = (* (b b))
    @and  = ((b (@fals c)) (b c))
    @main = root & @and ~ (@true (@fals root))
  ", 64);
  net.normal(&book);

  assert_snapshot!(show_net(&net), @"(* (b b))");
  assert_debug_snapshot!(net.rewrites(), @"9");
}

#[test]
fn test_church_mul() {
  let book = load_lang("church_mul.hvm");
  let (term, net) = hvm_lang_normal(book, 64);

  assert_snapshot!(show_net(&net), @"({2 ({2 b c} d) {3 (d e) (e {2 c f})}} (b f))");
  assert_snapshot!(term, @"λa λb (a (a (a (a (a (a b))))))");
  assert_debug_snapshot!(net.rewrites(), @"12");
}

#[test]
fn test_neg_fusion() {
  let book = load_lang("neg_fusion.hvm");
  let (term, net) = hvm_lang_normal(book, 512);

  assert_snapshot!(show_net(&net), @"(b (* b))");
  assert_snapshot!(term, @"λa λ* a");
  assert_debug_snapshot!(net.rewrites(), @"153");
}

#[test]
fn test_tree_alloc() {
  let book = load_lang("tree_alloc.hvm");
  let (term, net) = hvm_lang_normal(book, 512);

  assert_snapshot!(show_net(&net), @"(b (* b))");
  assert_snapshot!(term, @"λa λ* a");
  assert_debug_snapshot!(net.rewrites(), @"104");
}

#[test]
fn test_queue() {
  let book = load_lang("queue.hvm");
  let (term, net) = hvm_lang_normal(book, 512);

  assert_snapshot!(show_net(&net), @"((#1 (((#2 (((#3 ((* @7) b)) (* b)) c)) (* c)) d)) (* d))");
  assert_snapshot!(term, @"λa λ* ((a 1) λb λ* ((b 2) λc λ* ((c 3) λ* λd d)))");
  assert_debug_snapshot!(net.rewrites(), @"62");
}

#[test]
fn test_deref() {
  let (book, mut net) = parse_core("
    @ref = R & (x x) ~ (* R)
    @main = root & (* @ref) ~ (* root)
  ", 16);
  assert_eq!(net.heap.compact().len(), 1);

  net.expand(&book, net.heap.get_root());

  assert_eq!(net.heap.compact().len(), 3);
  let redexes = net.peek_current_redexes();
  assert_eq!(redexes.len(), 1);
}