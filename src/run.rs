// An efficient Interaction Combinator runtime
// ===========================================
// This file implements an efficient interaction combinator runtime. Nodes are represented by 2 aux
// ports (P1, P2), with the main port (P1) omitted. A separate vector, 'rdex', holds main ports,
// and, thus, tracks active pairs that can be reduced in parallel. Pointers are unboxed, meaning
// that ERAs, NUMs and REFs don't use any additional space. REFs lazily expand to closed nets when
// they interact with nodes, and are cleared when they interact with ERAs, allowing for constant
// space evaluation of recursive functions on Scott encoded datatypes.

use std::sync::mpsc::{Sender, Receiver, channel};

pub type Val = u32;

// Core terms.
#[repr(u8)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Tag {
  /// Variable to aux port 1
  VR1,
  /// Variable to aux port 2
  VR2,
  /// Redirect to aux port 1
  RD1,
  /// Redirect to aux port 2
  RD2,
  /// Lazy closed net
  REF,
  /// Unboxed eraser
  ERA,
  /// Unboxed number
  NUM,
  /// Binary numeric operation
  OP2,
  /// Unary numeric operation
  OP1,
  /// Numeric if-then-else(MATCH)
  MAT,
  /// Main port of con node(label 0)
  CT0,
  /// Main port of con node(label 1)
  CT1,
  /// Main port of con node(label 2)
  CT2,
  /// Main port of con node(label 3)
  CT3,
  /// Main port of con node(label 4)
  CT4,
  /// Main port of con node(label 5)
  CT5,
}

pub type NumericOp = u8;

// Numeric operations.
pub const USE: NumericOp = 0x0; // set-next-op
pub const ADD: NumericOp = 0x1; // addition
pub const SUB: NumericOp = 0x2; // subtraction
pub const MUL: NumericOp = 0x3; // multiplication
pub const DIV: NumericOp = 0x4; // division
pub const MOD: NumericOp = 0x5; // modulus
pub const EQ : NumericOp = 0x6; // equal-to
pub const NE : NumericOp = 0x7; // not-equal-to
pub const LT : NumericOp = 0x8; // less-than
pub const GT : NumericOp = 0x9; // greater-than
pub const AND: NumericOp = 0xA; // bitwise-and
pub const OR : NumericOp = 0xB; // bitwise-or
pub const XOR: NumericOp = 0xC; // bitwise-xor
pub const NOT: NumericOp = 0xD; // bitwise-not
pub const LSH: NumericOp = 0xE; // left-shift
pub const RSH: NumericOp = 0xF; // right-shift

// Root pointer.
pub const ERAS: Ptr = Ptr(0x0000_0000 | ERA as Val);
pub const ROOT: Ptr = Ptr(0x0000_0000 | VR2 as Val);
pub const NULL: Ptr = Ptr(0x0000_0000);

// An auxiliary port.
pub type Port = Val;
pub const P1: Port = 0;
pub const P2: Port = 1;

// A tagged pointer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ptr(pub Val);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Heap {
  data: Vec<(Ptr, Ptr)>,
  next: usize,
  used: usize,
  full: bool,
}

type Redex = (Ptr, Ptr);

// A interaction combinator net.
#[derive(Debug)]
pub struct Net {
  // redexes
  tx_redex: Sender<Redex>,
  rx_redex: Receiver<Redex>,
  redexes: Vec<Redex>,

  pub heap: Heap, // nodes
  pub anni: usize, // anni rewrites
  pub comm: usize, // comm rewrites
  pub eras: usize, // eras rewrites
  pub dref: usize, // dref rewrites
  pub oper: usize, // oper rewrites
}

// A compact closed net, used for dereferences.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Def {
  pub rdex: Vec<Redex>,
  pub node: Vec<(Ptr, Ptr)>,
}

// A map of id to definitions (closed nets).
pub struct Book {
  pub defs: Vec<Def>,
}

// Patterns for easier matching on tags
macro_rules! CTR{() => {CT0 | CT1 | CT2 | CT3 | CT4 | CT5}}
macro_rules! VAR{() => {VR1 | VR2}}
macro_rules! RED{() => {RD1 | RD2}}
macro_rules! OPS{() => {OP2 | OP1 | MAT}}
macro_rules! PRI{() => {REF | ERA | NUM | OPS!() | CTR!()}}

impl From<Tag> for u8 {
  #[inline(always)]
  fn from(tag: Tag) -> Self {
    tag as u8
  }
}

impl From<u8> for Tag {
  #[inline(always)]
  fn from(value: u8) -> Self {
    unsafe {
      std::mem::transmute(value)
    }
  }
}
pub use Tag::*;

impl Ptr {
  #[inline(always)]
  pub fn new(tag: Tag, val: Val) -> Self {
    Ptr((val << 4) | (tag as Val))
  }

  #[inline(always)]
  pub fn data(&self) -> Val {
    self.0
  }

  #[inline(always)]
  pub fn tag(&self) -> Tag {
    let tag_bits = (self.data() & 0xF) as u8;
    tag_bits.into()
  }

  #[inline(always)]
  pub fn val(&self) -> Val {
    (self.data() >> 4) as Val
  }

  #[inline(always)]
  pub fn is_nil(&self) -> bool {
    self.data() == 0
  }

  #[inline(always)]
  pub fn is_var(&self) -> bool {
    matches!(self.tag(), VAR!())
  }

  #[inline(always)]
  pub fn is_era(&self) -> bool {
    matches!(self.tag(), ERA)
  }

  #[inline(always)]
  pub fn is_ctr(&self) -> bool {
    matches!(self.tag(), CTR!())
  }

  #[inline(always)]
  pub fn is_ref(&self) -> bool {
    matches!(self.tag(), REF)
  }

  #[inline(always)]
  pub fn is_pri(&self) -> bool {
    matches!(self.tag(), PRI!())
  }

  #[inline(always)]
  pub fn is_num(&self) -> bool {
    matches!(self.tag(), NUM)
  }

  #[inline(always)]
  pub fn is_op1(&self) -> bool {
    matches!(self.tag(), OP1)
  }

  #[inline(always)]
  pub fn is_op2(&self) -> bool {
    matches!(self.tag(), OP2)
  }

  #[inline(always)]
  pub fn is_skp(&self) -> bool {
    matches!(self.tag(), ERA | NUM | REF)
  }

  #[inline(always)]
  pub fn is_mat(&self) -> bool {
    matches!(self.tag(), MAT)
  }

  #[inline(always)]
  /// All Ptrs that don't point to somewhere
  pub fn has_loc(&self) -> bool {
    matches!(self.tag(), VAR!() | OP2 | OP1 | MAT | CTR!())
  }

  // Can this redex be skipped (as an optimization)?
  #[inline(always)]
  pub fn can_skip(a: Ptr, b: Ptr) -> bool {
    matches!((a.tag(), b.tag()), (ERA | REF, ERA | REF))
  }
}

impl Book {
  #[inline(always)]
  pub fn new() -> Self {
    Book {
      defs: vec![Def::default(); 1 << 24],
    }
  }

  #[inline(always)]
  pub fn def(&mut self, id: Val, def: Def) {
    self.defs[id as usize] = def;
  }

  #[inline(always)]
  pub fn get(&self, id: Val) -> Option<&Def> {
    self.defs.get(id as usize)
  }
}

impl Heap {
  pub fn new(size: usize) -> Heap {
    Heap {
      data: vec![(NULL, NULL); size],
      next: 0,
      used: 0,
      full: false,
    }
  }

  #[inline(always)]
  pub fn alloc(&mut self) -> Val {
    self.next = (self.next + 1) % self.data.len();
    while !self.get(self.next as Val, P1).is_nil() {
      self.next = (self.next + 1) % self.data.len();
    }
    self.used += 1;
    self.next as Val
  }

  #[inline(always)]
  pub fn free(&mut self, index: Val) {
    self.used -= 1;
    self.set(index, P1, NULL);
    self.set(index, P2, NULL);
  }

  #[inline(always)]
  pub fn get(&self, index: Val, port: Port) -> Ptr {
    unsafe {
      let node = self.data.get_unchecked(index as usize);
      if port == P1 {
        return node.0;
      } else {
        return node.1;
      }
    }
  }

  #[inline(always)]
  pub fn set(&mut self, index: Val, port: Port, value: Ptr) {
    unsafe {
      let node = self.data.get_unchecked_mut(index as usize);
      if port == P1 {
        node.0 = value;
      } else {
        node.1 = value;
      }
    }
  }

  #[inline(always)]
  pub fn get_root(&self) -> Ptr {
    return self.get(0, P2);
  }

  #[inline(always)]
  pub fn set_root(&mut self, value: Ptr) {
    self.set(0, P2, value);
  }

  #[inline(always)]
  pub fn compact(&self) -> Vec<(Ptr, Ptr)> {
    let mut node = vec![];
    loop {
      let p1 = self.data[node.len()].0;
      let p2 = self.data[node.len()].1;
      if p1 != NULL || p2 != NULL {
        node.push((p1, p2));
      } else {
        break;
      }
    }
    return node;
  }

  fn extend_one(&mut self, p1: Ptr, p2: Ptr) -> Val {
    // TODO: maybe do both lines atomically?
    let index = self.data.len() as Val;
    self.data.push((p1, p2));
    index
  }
}


impl Net {
  // Creates an empty net with given size.
  pub fn new(size: usize) -> Self {
    let (tx_redex, rx_redex) = channel();
    Net {
      tx_redex,
      rx_redex,
      redexes: vec![],
      heap: Heap::new(size),
      anni: 0,
      comm: 0,
      eras: 0,
      dref: 0,
      oper: 0,
    }
  }

  // Creates a net and boots from a REF.
  pub fn boot(&mut self, root_id: Val) {
    self.heap.set_root(Ptr::new(REF, root_id));
  }

  // Converts to a def.
  pub fn to_def(self) -> Def {
    Def { rdex: self.peek_current_redexes(), node: self.heap.compact() }
  }

  // Reads back from a def.
  pub fn from_def(def: Def) -> Self {
    let mut net = Net::new(def.node.len());
    for (i, &(p1, p2)) in def.node.iter().enumerate() {
      net.heap.set(i as Val, P1, p1);
      net.heap.set(i as Val, P2, p2);
    }
    for r in def.rdex {
      net.put_redex(r);
    }
    net
  }

  // Gets a pointer's target.
  #[inline(always)]
  pub fn get_target(&self, ptr: Ptr) -> Ptr {
    self.heap.get(ptr.val(), ptr.0 & 1)
  }

  // Sets a pointer's target.
  #[inline(always)]
  pub fn set_target(&mut self, ptr: Ptr, val: Ptr) {
    self.heap.set(ptr.val(), ptr.0 & 1, val)
  }

  // Links two pointers, forming a new wire.
  pub fn link(&mut self, a: Ptr, b: Ptr) {
    // Creates redex A-B
    if a.is_pri() && b.is_pri() {
      if Ptr::can_skip(a, b) {
        self.eras += 1;
      } else {
        self.put_redex((a, b));
      }
      return;
    }
    // Substitutes A
    if a.is_var() {
      self.set_target(a, b);
    }
    // Substitutes B
    if b.is_var() {
      self.set_target(b, a);
    }
  }

  // Performs an interaction over a redex.
  pub fn interact(&mut self, book: &Book, a: Ptr, b: Ptr) {
    let mut a = a;
    let mut b = b;

    // Dereference A or B
    match (a.tag(), b.tag()) {
      (REF, OPS!() | CTR!()) => a = self.deref(book, a, b),
      (OPS!() | CTR!(), REF) => b = self.deref(book, b, a),
      _ => {}
    }

    match (a.tag(), b.tag()) {
      // CTR-CTR when same labels
      (CT0, CT0) | (CT1, CT1) | (CT2, CT2) | (CT3, CT3) | (CT4, CT4) | (CT5, CT5)
        => self.anni(a, b),
      // CTR-CTR when different labels
      (CTR!(), CTR!()) => self.comm(a, b),
      (CTR!(), ERA)    => self.era2(a),
      (ERA, CTR!())    => self.era2(b),
      (REF, ERA)       => self.eras += 1,
      (ERA, REF)       => self.eras += 1,
      (ERA, ERA)       => self.eras += 1,
      (VAR!(), _)      => self.link(a, b),
      (_, VAR!())      => self.link(b, a),
      (CTR!(), NUM)    => self.copy(a, b),
      (NUM, CTR!())    => self.copy(b, a),
      (NUM, ERA)       => self.eras += 1,
      (ERA, NUM)       => self.eras += 1,
      (NUM, NUM)       => self.eras += 1,
      (OP2, NUM)       => self.op2n(a, b),
      (NUM, OP2)       => self.op2n(b, a),
      (OP1, NUM)       => self.op1n(a, b),
      (NUM, OP1)       => self.op1n(b, a),
      (OP2, CTR!())    => self.comm(a, b),
      (CTR!(), OP2)    => self.comm(b, a),
      (OP1, CTR!())    => self.pass(a, b),
      (CTR!(), OP1)    => self.pass(b, a),
      (OP2, ERA)       => self.era2(a),
      (ERA, OP2)       => self.era2(b),
      (OP1, ERA)       => self.era1(a),
      (ERA, OP1)       => self.era1(b),
      (MAT, NUM)       => self.mtch(a, b),
      (NUM, MAT)       => self.mtch(b, a),
      (MAT, CTR!())    => self.comm(a, b),
      (CTR!(), MAT)    => self.comm(b, a),
      (MAT, ERA)       => self.era2(a),
      (ERA, MAT)       => self.era2(b),

      // because of the deref above this match
      // we know that A and B are not REFs
      (REF, _) => unreachable!(),
      (_, REF) => unreachable!(),

      // undefined numerical interactions resulting from a sort of "type error"
      (OPS!(), OPS!()) => unreachable!(),

      // TODO: this will change when we implement the multi-threaded version
      (RED!(), _) => unreachable!(),
      (_, RED!()) => unreachable!(),
    };
  }

  pub fn conn(&mut self, a: Ptr, b: Ptr) {
    self.anni += 1;
    self.link(self.heap.get(a.val(), P2), self.heap.get(b.val(), P2));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn anni(&mut self, a: Ptr, b: Ptr) {
    self.anni += 1;
    self.link(self.heap.get(a.val(), P1), self.heap.get(b.val(), P1));
    self.link(self.heap.get(a.val(), P2), self.heap.get(b.val(), P2));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn comm(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    let (loc1, loc2, loc3, loc4) = (self.heap.alloc(), self.heap.alloc(), self.heap.alloc(), self.heap.alloc());
    self.link(self.heap.get(a.val(), P1), Ptr::new(b.tag(), loc1));
    self.link(self.heap.get(b.val(), P1), Ptr::new(a.tag(), loc3));
    self.link(self.heap.get(a.val(), P2), Ptr::new(b.tag(), loc2));
    self.link(self.heap.get(b.val(), P2), Ptr::new(a.tag(), loc4));
    self.heap.set(loc1, P1, Ptr::new(VR1, loc3));
    self.heap.set(loc1, P2, Ptr::new(VR1, loc4));
    self.heap.set(loc2, P1, Ptr::new(VR2, loc3));
    self.heap.set(loc2, P2, Ptr::new(VR2, loc4));
    self.heap.set(loc3, P1, Ptr::new(VR1, loc1));
    self.heap.set(loc3, P2, Ptr::new(VR1, loc2));
    self.heap.set(loc4, P1, Ptr::new(VR2, loc1));
    self.heap.set(loc4, P2, Ptr::new(VR2, loc2));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn pass(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    let (loc1, loc2, loc3) = (self.heap.alloc(), self.heap.alloc(), self.heap.alloc());
    self.link(self.heap.get(a.val(), P2), Ptr::new(b.tag(), loc1));
    self.link(self.heap.get(b.val(), P1), Ptr::new(a.tag(), loc2));
    self.link(self.heap.get(b.val(), P2), Ptr::new(a.tag(), loc3));
    self.heap.set(loc1, P1, Ptr::new(VR2, loc2));
    self.heap.set(loc1, P2, Ptr::new(VR2, loc3));
    self.heap.set(loc2, P1, self.heap.get(a.val(), P1));
    self.heap.set(loc2, P2, Ptr::new(VR1, loc1));
    self.heap.set(loc3, P1, self.heap.get(a.val(), P1));
    self.heap.set(loc3, P2, Ptr::new(VR2, loc1));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn copy(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    self.link(self.heap.get(a.val(), P1), b);
    self.link(self.heap.get(a.val(), P2), b);
    self.heap.free(a.val());
  }

  pub fn era2(&mut self, a: Ptr) {
    self.eras += 1;
    self.link(self.heap.get(a.val(), P1), ERAS);
    self.link(self.heap.get(a.val(), P2), ERAS);
    self.heap.free(a.val());
  }

  pub fn era1(&mut self, a: Ptr) {
    self.eras += 1;
    self.link(self.heap.get(a.val(), P2), ERAS);
    self.heap.free(a.val());
  }


  pub fn op2n(&mut self, a: Ptr, b: Ptr) {
    self.oper += 1;
    let mut p1 = self.heap.get(a.val(), P1);
    // Optimization: perform chained ops at once
    if p1.is_num() {
      let mut rt = b.val();
      let mut p2 = self.heap.get(a.val(), P2);
      loop {
        self.oper += 1;
        rt = self.prim(rt, p1.val());
        // If P2 is OP2, keep looping
        if p2.is_op2() {
          p1 = self.heap.get(p2.val(), P1);
          if p1.is_num() {
            p2 = self.heap.get(p2.val(), P2);
            self.oper += 1; // since OP1 is skipped
            continue;
          }
        }
        // If P2 is OP1, flip args and keep looping
        if p2.is_op1() {
          let tmp = rt;
          rt = self.heap.get(p2.val(), P1).val();
          p1 = Ptr::new(NUM, tmp);
          p2 = self.heap.get(p2.val(), P2);
          continue;
        }
        break;
      }
      self.link(Ptr::new(NUM, rt), p2);
      return;
    }
    self.heap.set(a.val(), P1, b);
    self.link(Ptr::new(OP1, a.val()), p1);
  }

  pub fn op1n(&mut self, a: Ptr, b: Ptr) {
    self.oper += 1;
    let p1 = self.heap.get(a.val(), P1);
    let p2 = self.heap.get(a.val(), P2);
    let v0 = p1.val() as u32;
    let v1 = b.val() as u32;
    let v2 = self.prim(v0, v1);
    self.link(Ptr::new(NUM, v2), p2);
    self.heap.free(a.val());
  }

  pub fn prim(&mut self, a: u32, b: u32) -> u32 {
    let a_opr = (a >> 24) & 0xF;
    let b_opr = (b >> 24) & 0xF; // not used yet
    let a_val = a & 0xFFFFFF;
    let b_val = b & 0xFFFFFF;
    match a_opr as NumericOp {
      USE => { ((a_val & 0xF) << 24) | b_val }
      ADD => { (a_val.wrapping_add(b_val)) & 0xFFFFFF }
      SUB => { (a_val.wrapping_sub(b_val)) & 0xFFFFFF }
      MUL => { (a_val.wrapping_mul(b_val)) & 0xFFFFFF }
      DIV if b_val == 0 => { 0xFFFFFF }
      DIV => { (a_val.wrapping_div(b_val)) & 0xFFFFFF }
      MOD => { (a_val.wrapping_rem(b_val)) & 0xFFFFFF }
      EQ  => { ((a_val == b_val) as Val) & 0xFFFFFF }
      NE  => { ((a_val != b_val) as Val) & 0xFFFFFF }
      LT  => { ((a_val < b_val) as Val) & 0xFFFFFF }
      GT  => { ((a_val > b_val) as Val) & 0xFFFFFF }
      AND => { (a_val & b_val) & 0xFFFFFF }
      OR  => { (a_val | b_val) & 0xFFFFFF }
      XOR => { (a_val ^ b_val) & 0xFFFFFF }
      NOT => { (!b_val) & 0xFFFFFF }
      LSH => { (a_val << b_val) & 0xFFFFFF }
      RSH => { (a_val >> b_val) & 0xFFFFFF }
      _   => { unreachable!() }
    }
  }

  pub fn mtch(&mut self, a: Ptr, b: Ptr) {
    self.oper += 1;
    let p1 = self.heap.get(a.val(), P1); // branch
    let p2 = self.heap.get(a.val(), P2); // return
    if b.val() == 0 {
      let loc = self.heap.alloc();
      self.heap.set(loc, P2, ERAS);
      self.link(p1, Ptr::new(CT0, loc));
      self.link(p2, Ptr::new(VR1, loc));
      self.heap.free(a.val());
    } else {
      let (loc1, loc2) = (self.heap.alloc(), self.heap.alloc());
      self.heap.set(loc1, P1, ERAS);
      self.heap.set(loc1, P2, Ptr::new(CT0, loc2));
      self.heap.set(loc2, P1, Ptr::new(NUM, b.val() - 1));
      self.link(p1, Ptr::new(CT0, loc1));
      self.link(p2, Ptr::new(VR2, loc2));
      self.heap.free(a.val());
    }
  }

  // Expands a closed net.
  #[inline(always)]
  pub fn deref(&mut self, book: &Book, ptr: Ptr, parent: Ptr) -> Ptr {
    self.dref += 1;
    let mut root = ptr;
    // FIXME: change "while" to "if" once lang prevents refs from returning refs
    while root.is_ref() {
      // Load the closed net.
      let mut def_net = unsafe { book.defs.get_unchecked((root.val() as usize) & 0xFFFFFF) }.clone();
      if def_net.node.len() == 0 {
        continue;
      }


      // TODO: Reuse Vec between calls (thread-local memory)
      let mut locs = vec![parent.val()];
      locs.extend(
        def_net.node.iter()
          .skip(1) // skip root
          .map(|_| self.heap.alloc())
      );

      let adjust_ptr = #[inline(always)] |ptr: &mut Ptr| {
        if ptr.has_loc() {
          *ptr = Ptr::new(ptr.tag(), locs[ptr.val() as usize]);
        }
      };


      // Adjust all nodes
      (_, root) = def_net.node[0];
      adjust_ptr(&mut root);
      for (p1, p2) in &mut def_net.node[1..] {
        adjust_ptr(p1);
        adjust_ptr(p2);
      }

      // Load nodes and redexes
      for (loc, (p1, p2)) in locs.clone().into_iter().skip(1).zip(&def_net.node[1..]) {
        // if p1.val() == locs[0] {
        //   self.set_target(*p1, Ptr::new(p1.tag(), loc));
        // }
        // if p2.val() == locs[0] {
        //   self.set_target(*p2, Ptr::new(p2.tag(), loc));
        // }
        self.heap.set(loc, P1, *p1);
        self.heap.set(loc, P2, *p2);
      }
      for (a, b) in &def_net.rdex {
        self.put_redex((*a, *b));
      }
      // if root.is_var() {
      //   self.set_target(parent, root);
      // }
    }
    return root;
  }

  // Reduces all redexes.
  pub fn reduce(&mut self, book: &Book) {
    // let mut redex_snapshot = std::mem::take(&mut self.redexes);
    // while redex_snapshot.len() > 0 {
    //   for (a, b) in redex_snapshot.drain(..) {
    //     self.interact(book, a, b);
    //   }
    //   std::mem::swap(&mut self.redexes, &mut redex_snapshot);
    // }
    while let Some((a, b)) = self.get_next_redex() {
      self.interact(book, a, b);
    }
  }

  // Reduce a net to normal form.
  pub fn normal(&mut self, book: &Book) {
    self.expand(book, ROOT);
    while self.has_redex() {
      self.reduce(book);
      self.expand(book, ROOT);
    }
  }

  // Expands heads.
  pub fn expand(&mut self, book: &Book, dir: Ptr) {
    let ptr = self.get_target(dir);
    if ptr.is_ctr() {
      self.expand(book, Ptr::new(VR1, ptr.val()));
      self.expand(book, Ptr::new(VR2, ptr.val()));
    } else if ptr.is_ref() {
      let exp = self.deref(book, ptr, dir);
      self.set_target(dir, exp);
    }
  }

  // Total rewrite count.
  pub fn rewrites(&self) -> usize {
    self.anni + self.comm + self.eras + self.dref + self.oper
  }

  fn has_redex(&self) -> bool {
    self.redexes.len() > 0
  }

  pub(crate) fn get_next_redex(&mut self) -> Option<Redex> {
    // self.rx_redex.recv().ok()
    self.redexes.pop()
  }

  pub fn peek_current_redexes(&self) -> Vec<Redex> {
    // let redexes: Vec<_> = self.rx_redex.try_iter().collect();
    // for r in redexes.clone() {
    //   self.tx_redex.send(r).unwrap();
    // }
    // redexes
    self.redexes.clone()
  }

  pub(crate) fn put_redex(&mut self, r: Redex) {
    // self.tx_redex.send(r).unwrap();
    self.redexes.push(r);
  }
}
