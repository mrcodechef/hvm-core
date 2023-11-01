// An efficient Interaction Combinator runtime
// ===========================================
// This file implements an efficient interaction combinator runtime. Nodes are represented by 2 aux
// ports (P1, P2), with the main port (P1) omitted. A separate vector, 'rdex', holds main ports,
// and, thus, tracks active pairs that can be reduced in parallel. Pointers are unboxed, meaning
// that ERAs, NUMs and REFs don't use any additional space. REFs lazily expand to closed nets when
// they interact with nodes, and are cleared when they interact with ERAs, allowing for constant
// space evaluation of recursive functions on Scott encoded datatypes.

pub type Val = u32;

// Core terms.
#[repr(u8)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum Tag {
  /// Variable to aux port
  VAR,
  /// Redirect to aux port
  RDR,
  /// Lazy closed net
  REF,
  /// Unboxed eraser
  ERA,
  /// Unboxed number
  NUM,
  /// First node of numeric operation
  OP2,
  /// Second node of numeric operation
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

// Root address
pub const ROOT_PORT: Val = 0 + P2;
pub const FIRST_PORT: Val = 2 + P1;

// Root pointer.
pub const ERAS: Ptr = Ptr::new(ERA, 0x0000_0000);
pub const ROOT: Ptr = Ptr::new(VAR, ROOT_PORT);
pub const NULL: Ptr = Ptr(0);
// An auxiliary port.
pub type Port = Val;
pub const P1: Port = 0;
pub const P2: Port = 1;


// A tagged pointer.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ptr(pub Val);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Heap {
  data: Vec<Ptr>,
  next: usize,
  used: usize,
  full: bool,
}

// A interaction combinator net.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Net {
  pub rdex: Vec<(Ptr,Ptr)>, // redexes
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
  pub rdex: Vec<(Ptr, Ptr)>,
  pub port: Vec<Ptr>,
}

// A map of id to definitions (closed nets).
pub struct Book {
  pub defs: Vec<Def>,
}

// Patterns for easier matching on tags
macro_rules! CTR{() => {CT0 | CT1 | CT2 | CT3 | CT4 | CT5}}
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

impl Tag {
  #[inline(always)]
  pub fn ctr(lab: u8) -> Self {
    (Self::CT0 as u8 + lab).into()
  }
}

impl Ptr {
  #[inline(always)]
  pub const fn new(tag: Tag, val: Val) -> Self {
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
    matches!(self.tag(), VAR)
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
  pub fn has_loc(&self) -> bool {
    matches!(self.tag(), VAR | OP2 | OP1 | MAT | CTR!())
  }

  #[inline(always)]
  pub fn adjust(&self, loc: Val) -> Ptr {
    return Ptr::new(self.tag(), self.val() + if self.has_loc() { loc - FIRST_PORT } else { 0 });
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
    return Heap {
      data: vec![NULL; size],
      next: FIRST_PORT as usize,
      used: 0,
      full: false,
    };
  }

  #[inline(always)]
  // Allocs `size` ports in the heap
  pub fn alloc(&mut self, size: usize) -> Val {
    if size == 0 {
      return 0;
    } else if !self.full && self.next + size <= self.data.len() {
      self.used += size;
      self.next += size;
      return (self.next - size) as Val;
    } else {
      self.full = true;
      let mut space = 0;
      loop {
        if self.next >= self.data.len() {
          space = 0;
          self.next = FIRST_PORT as usize;
        }
        if self.get(self.next as Val).is_nil() {
          space += 1;
        } else {
          space = 0;
        }
        self.next += 1;
        if space == size {
          self.used += size;
          return (self.next - space) as Val;
        }
      }
    }
  }

  #[inline(always)]
  pub fn free(&mut self, index: Val) {
    self.used -= 2;
    self.set(index + P1, NULL);
    self.set(index + P2, NULL);
  }

  #[inline(always)]
  pub fn lock(&self, index: Val) {
    return;
  }

  #[inline(always)]
  pub fn unlock(&self, index: Val) {
    return;
  }

  #[inline(always)]
  pub fn get(&self, index: Val) -> Ptr {
    unsafe {
      *self.data.get_unchecked(index as usize)
    }
  }

  #[inline(always)]
  pub fn set(&mut self, index: Val, value: Ptr) {
    unsafe {
      let node = self.data.get_unchecked_mut(index as usize);
      *node = value;
    }
  }

  #[inline(always)]
  pub fn get_root(&self) -> Ptr {
    return self.get(ROOT_PORT);
  }

  #[inline(always)]
  pub fn set_root(&mut self, value: Ptr) {
    self.set(ROOT_PORT, value);
  }

  #[inline(always)]
  pub fn compact(&self) -> Vec<Ptr> {
    let mut node = vec![];
    loop {
      let p1 = self.data[node.len() + P1 as usize];
      let p2 = self.data[node.len() + P2 as usize];
      if p1 != NULL || p2 != NULL {
        node.push(p1);
        node.push(p2);
      } else {
        break;
      }
    }
    return node;
  }
}

impl Net {
  // Creates an empty net with given size.
  pub fn new(size: usize) -> Self {
    Net {
      rdex: vec![],
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
    Def { rdex: self.rdex, port: self.heap.compact() }
  }

  // Reads back from a def.
  pub fn from_def(def: Def) -> Self {
    let mut net = Net::new(def.port.len());
    for (i, &p) in def.port.iter().enumerate() {
      net.heap.set(i as Val, p);
    }
    net.rdex = def.rdex;
    net
  }

  // Gets a pointer's target.
  #[inline(always)]
  pub fn get_target(&self, ptr: Ptr) -> Ptr {
    self.heap.get(ptr.val())
  }

  // Sets a pointer's target.
  #[inline(always)]
  pub fn set_target(&mut self, ptr: Ptr, val: Ptr) {
    self.heap.set(ptr.val(), val)
  }

  // Links two pointers, forming a new wire.
  pub fn link(&mut self, a: Ptr, b: Ptr) {
    // Creates redex A-B
    if a.is_pri() && b.is_pri() {
      if Ptr::can_skip(a, b) {
        self.eras += 1;
      } else {
        self.rdex.push((a, b));
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
      (VAR, _)         => self.link(a, b),
      (_, VAR)         => self.link(b, a),
      (CTR!(), NUM)    => self.copy(a, b),
      (NUM, CTR!())    => self.copy(b, a),
      (NUM, ERA)       => self.eras += 1,
      (ERA, NUM)       => self.eras += 1,
      (NUM, NUM)       => self.eras += 1,
      (OP2, NUM)       => self.op2n(a, b),
      (NUM, OP2)       => self.op2n(b, a),
      (OP1, NUM)       => self.op1n(a, b),
      (NUM, OP1)       => self.op1n(b, a),
      (OP2, OP1)       => self.opfn(a, b),
      (OP1, OP2)       => self.opfn(b, a),
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
      (OPS!(), _) => unreachable!(),

      // TODO: this will change when we implement the multi-threaded version
      (RDR, _) => unreachable!(),
      (_, RDR) => unreachable!(),
    };
  }

  pub fn conn(&mut self, a: Ptr, b: Ptr) {
    self.anni += 1;
    self.link(self.heap.get(a.val() + P2), self.heap.get(b.val() + P2));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn anni(&mut self, a: Ptr, b: Ptr) {
    self.anni += 1;
    self.link(self.heap.get(a.val() + P1), self.heap.get(b.val() + P1));
    self.link(self.heap.get(a.val() + P2), self.heap.get(b.val() + P2));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn comm(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    let loc = self.heap.alloc(8);
    // Link 4 main ports of the new nodes
    self.link(self.heap.get(a.val() + P1), Ptr::new(b.tag(), loc + 0));
    self.link(self.heap.get(b.val() + P1), Ptr::new(a.tag(), loc + 4));
    self.link(self.heap.get(a.val() + P2), Ptr::new(b.tag(), loc + 2));
    self.link(self.heap.get(b.val() + P2), Ptr::new(a.tag(), loc + 6));
    // List the 8 aux ports
    self.heap.set(loc + 0, Ptr::new(VAR, loc + 4));
    self.heap.set(loc + 1, Ptr::new(VAR, loc + 6));
    self.heap.set(loc + 2, Ptr::new(VAR, loc + 5));
    self.heap.set(loc + 3, Ptr::new(VAR, loc + 7));
    self.heap.set(loc + 4, Ptr::new(VAR, loc + 0));
    self.heap.set(loc + 5, Ptr::new(VAR, loc + 2));
    self.heap.set(loc + 6, Ptr::new(VAR, loc + 1));
    self.heap.set(loc + 7, Ptr::new(VAR, loc + 3));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn pass(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    let loc = self.heap.alloc(6);
    self.link(self.heap.get(a.val() + P2), Ptr::new(b.tag(), loc+0));
    self.link(self.heap.get(b.val() + P1), Ptr::new(a.tag(), loc+2));
    self.link(self.heap.get(b.val() + P2), Ptr::new(a.tag(), loc+4));
    self.heap.set(loc + 0, Ptr::new(VAR, loc+3));
    self.heap.set(loc + 1, Ptr::new(VAR, loc+5));
    self.heap.set(loc + 2, self.heap.get(a.val()+P1));
    self.heap.set(loc + 3, Ptr::new(VAR, loc+0));
    self.heap.set(loc + 4, self.heap.get(a.val()+P1));
    self.heap.set(loc + 5, Ptr::new(VAR, loc+1));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn copy(&mut self, a: Ptr, b: Ptr) {
    self.comm += 1;
    self.link(self.heap.get(a.val() + P1), b);
    self.link(self.heap.get(a.val() + P2), b);
    self.heap.free(a.val());
  }

  pub fn era2(&mut self, a: Ptr) {
    self.eras += 1;
    self.link(self.heap.get(a.val() + P1), ERAS);
    self.link(self.heap.get(a.val() + P2), ERAS);
    self.heap.free(a.val());
  }

  pub fn era1(&mut self, a: Ptr) {
    self.eras += 1;
    self.link(self.heap.get(a.val() + P2), ERAS);
    self.heap.free(a.val());
  }


  pub fn op2n(&mut self, a: Ptr, b: Ptr) {
    // Converts `a` from an OP2 into an OP1, storing `b` in port 1.
    self.oper += 1;
    let p1 = self.heap.get(a.val() + P1);
    // Optimization: try to do all steps of calling numeric function at once
    if p1.is_num() {
      self.oper += 1;
      let p2 = self.heap.get(a.val() + P2);
      if p2.is_op1() {
        // Do all the 3 steps of the operation
        self.oper += 1;
        let f = self.heap.get(p2.val() + P1);
        let res = self.prim(f.val(), b.val(), p1.val());
        self.link(self.heap.get(p2.val() + P2), Ptr::new(NUM, res));
        self.heap.free(a.val());
        self.heap.free(p2.val());
      } else {
         // Do just the first 2 steps
         self.heap.set(a.val() + P1, b);
         self.heap.set(a.val() + P2, p1);
         self.link(Ptr::new(OP2, a.val()), p2);
      }
    } else {
      // Only do the first step, the actual reduction of OP2 ~ NUM
      self.link(Ptr::new(OP1, a.val()), p1);
      self.heap.set(a.val() + P1, b);
    }
  }

  pub fn op1n(&mut self, a: Ptr, b: Ptr) {
    // Converts `a` from OP1 into OP2, storing NUM `b` in port 2.
    self.oper += 1;
    let p1 = self.heap.get(a.val() + P1);
    let p2 = self.heap.get(a.val() + P2);
    // Optimization: Try to complete the whole numeric operation
    if p2.is_op1() {
      // Do the remaining step as well
      self.oper += 1;
      let f = self.heap.get(p2.val() + P1);
      let res = self.prim(f.val(), p1.val(), b.val());
      self.link(self.heap.get(p2.val() + P2), Ptr::new(NUM, res));
      self.heap.free(a.val());
      self.heap.free(p2.val());
    } else {
       // Do just the OP1 ~ NUM reduction
       self.heap.set(a.val() + P2, b);
       self.link(Ptr::new(OP2, a.val()), p2);
    }
  }

  pub fn opfn(&mut self, a: Ptr, b: Ptr) {
    // Executes a binary numeric function.
    // `a` is an OP2 holding the 2 args, `b` is an OP1 with the function in port 1.
    self.oper += 1;
    let v1 = self.heap.get(a.val() + P1).val();
    let v2 = self.heap.get(a.val() + P2).val();
    let f = self.heap.get(b.val() + P1).val();
    let res = self.prim(f, v1, v2);
    self.link(self.heap.get(b.val() + P2), Ptr::new(NUM, res));
    self.heap.free(a.val());
    self.heap.free(b.val());
  }

  pub fn prim(&mut self, f: Val, a: Val, b: Val) -> Val {
    let a_val = a & 0xFFFFFF;
    let b_val = b & 0xFFFFFF;
    match f as u8 {
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
    let p1 = self.heap.get(a.val() + P1); // branch
    let p2 = self.heap.get(a.val() + P2); // return
    if b.val() == 0 {
      let loc = self.heap.alloc(2);
      self.heap.set(loc+0+P2, ERAS);
      self.link(p1, Ptr::new(CT0, loc+0));
      self.link(p2, Ptr::new(VAR, loc+0));
      self.heap.free(a.val());
    } else {
      let loc = self.heap.alloc(4);
      self.heap.set(loc+0+P1, ERAS);
      self.heap.set(loc+0+P2, Ptr::new(CT0, loc + 2));
      self.heap.set(loc+2+P1, Ptr::new(NUM, b.val() - 1));
      self.link(p1, Ptr::new(CT0, loc+0));
      self.link(p2, Ptr::new(VAR, loc+3));
      self.heap.free(a.val());
    }
  }

  // Expands a closed net.
  #[inline(always)]
  pub fn deref(&mut self, book: &Book, ptr: Ptr, parent: Ptr) -> Ptr {
    self.dref += 1;
    let mut ptr = ptr;
    // FIXME: change "while" to "if" once lang prevents refs from returning refs
    while ptr.is_ref() {
      // Load the closed net.
      let got = unsafe { book.defs.get_unchecked((ptr.val() as usize) & 0xFFFFFF) };
      if got.port.len() > 0 {
        let len = got.port.len() - FIRST_PORT as usize;
        let loc = self.heap.alloc(len);
        // Load nodes, adjusted.
        for i in 0..len as Val {
          unsafe {
            let p = got.port.get_unchecked((FIRST_PORT + i) as usize).adjust(loc);
            self.heap.set(loc + i, p);
          }
        }
        for r in &got.rdex {
          let p1 = r.0.adjust(loc);
          let p2 = r.1.adjust(loc);
          self.rdex.push((p1, p2));
        }
        // Load root, adjusted.
        ptr = got.port[ROOT_PORT as usize].adjust(loc);
        // Link root.
        if ptr.is_var() {
          self.set_target(ptr, parent);
        }
      }
    }
    return ptr;
  }

  // Reduces all redexes.
  pub fn reduce(&mut self, book: &Book) {
    let mut rdex: Vec<(Ptr, Ptr)> = vec![];
    std::mem::swap(&mut self.rdex, &mut rdex);
    while rdex.len() > 0 {
      for (a, b) in &rdex {
        self.interact(book, *a, *b);
      }
      rdex.clear();
      std::mem::swap(&mut self.rdex, &mut rdex);
    }
  }

  pub fn reduce2(&mut self, book: &Book) {
    while !self.rdex.is_empty() {
      let (a, b) = self.rdex.remove(0);
      self.interact(book, a, b);
      //eprintln!("{}\n", crate::ast::show_runtime_net(&self));
    }
  }

  // Reduce a net to normal form.
  pub fn normal(&mut self, book: &Book) {
    self.expand(book, ROOT);
    while self.rdex.len() > 0 {
      self.reduce(book);
      self.expand(book, ROOT);
    }
  }

  // Expands heads.
  pub fn expand(&mut self, book: &Book, dir: Ptr) {
    let ptr = self.get_target(dir);
    if ptr.is_ctr() {
      self.expand(book, Ptr::new(VAR, ptr.val()+P1));
      self.expand(book, Ptr::new(VAR, ptr.val()+P2));
    } else if ptr.is_ref() {
      let exp = self.deref(book, ptr, dir);
      self.set_target(dir, exp);
    }
  }

  // Total rewrite count.
  pub fn rewrites(&self) -> usize {
    self.anni + self.comm + self.eras + self.dref + self.oper
  }
}
