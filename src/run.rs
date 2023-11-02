// An efficient Interaction Combinator runtime
// ===========================================
// This file implements an efficient interaction combinator runtime. Nodes are represented by 2 aux
// ports (P1, P2), with the main port (P1) omitted. A separate vector, 'rdex', holds main ports,
// and, thus, tracks active pairs that can be reduced in parallel. Pointers are unboxed, meaning
// that ERAs, NUMs and REFs don't use any additional space. REFs lazily expand to closed nets when
// they interact with nodes, and are cleared when they interact with ERAs, allowing for constant
// space evaluation of recursive functions on Scott encoded datatypes.

use crossbeam_queue::SegQueue;
use std::sync::{mpsc::{Sender, Receiver, channel}, Mutex, atomic::{AtomicUsize, Ordering}, MutexGuard};

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

#[derive(Debug)]
pub struct Heap {
  data: Vec<Mutex<AuxPorts>>,
  next: AtomicUsize,
  used: AtomicUsize,
  full: bool,
}

type Redex = (Ptr, Ptr);

// A interaction combinator net.
#[derive(Debug)]
pub struct Net {
  redexes: SegQueue<Redex>, // redexes
  pub heap: Heap, // nodes
  pub anni: AtomicUsize, // anni rewrites
  pub comm: AtomicUsize, // comm rewrites
  pub eras: AtomicUsize, // eras rewrites
  pub dref: AtomicUsize, // dref rewrites
  pub oper: AtomicUsize, // oper rewrites
}

impl Net {
  pub fn anni_(&self) -> usize {
    self.anni.load(Ordering::Relaxed)
  }

  pub fn comm_(&self) -> usize {
    self.comm.load(Ordering::Relaxed)
  }

  pub fn eras(&self) -> usize {
    self.eras.load(Ordering::Relaxed)
  }

  pub fn dref(&self) -> usize {
    self.dref.load(Ordering::Relaxed)
  }

  pub fn oper(&self) -> usize {
    self.oper.load(Ordering::Relaxed)
  }
}

// A compact closed net, used for dereferences.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct Def {
  pub rdex: Vec<Redex>,
  pub node: Vec<AuxPorts>,
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

type AuxPorts = (Ptr, Ptr);
type AcquiredNode<'a> = MutexGuard<'a, AuxPorts>;

pub fn set_port(node: &mut AuxPorts, port: Port, val: Ptr) {
  if port == P1 {
    node.0 = val;
  } else {
    node.1 = val;
  }
}

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
      data: (0 .. size).map(|_| Mutex::new((NULL, NULL))).collect(),
      next: AtomicUsize::new(0),
      used: AtomicUsize::new(0),
      full: false,
    }
  }

  #[inline(always)]
  pub fn alloc(&self) -> (AcquiredNode, Val) {
    loop {
      // Increment next, wrapping around
      let mut cur = self.next.load(Ordering::Relaxed);
      let n = self.data.len();
      loop {
        let new = (cur + 1) % n;
        match self.next.compare_exchange_weak(cur, new, Ordering::Relaxed, Ordering::Relaxed) {
          Ok(_) => break,
          Err(x) => cur = x,
        }
      }

      // Check if it's free
      // Note: A thread can block on .lock() if more than self.data.len() threads call `alloc`` at the same time
      let lock_to_free_slot = unsafe { self.data.get_unchecked(cur as usize) }.lock().unwrap();
      if lock_to_free_slot.0.is_nil() {
        self.used.fetch_add(1, Ordering::Relaxed);
        return (lock_to_free_slot, cur as Val);
      }
    }
  }

  #[inline(always)]
  pub fn free(&self, mut node: AcquiredNode) {
    self.used.fetch_sub(1, Ordering::Relaxed);
    *node = (NULL, NULL);
  }

  // #[inline(always)]
  // pub fn get(&self, index: Val, port: Port) -> Ptr {
  //   let node = unsafe { self.data.get_unchecked(index as usize) }.lock().unwrap();
  //   if port == P1 {
  //     node.0
  //   } else {
  //     node.1
  //   }
  // }

  #[inline(always)]
  pub fn acquire_node(&self, index: Val) -> AcquiredNode {
    unsafe { self.data.get_unchecked(index as usize) }.lock().unwrap()
  }

  #[inline(always)]
  pub fn try_acquire_node(&self, index: Val) -> Option<AcquiredNode> {
    unsafe { self.data.get_unchecked(index as usize) }.try_lock().ok()
  }

  #[inline(always)]
  pub fn decompose(&self, index: Val) -> AuxPorts {
    let node = self.acquire_node(index);
    let (n0, n1) = *node;
    self.free(node);
    (n0, n1)
  }

  // #[inline(always)]
  // pub fn set(&self, index: Val, port: Port, value: Ptr) {
  //   let mut node = unsafe { self.data.get_unchecked(index as usize) }.lock().unwrap();
  //   if port == P1 {
  //     node.0 = value;
  //   } else {
  //     node.1 = value;
  //   }
  // }

  #[inline(always)]
  pub fn get_root(&self) -> Ptr {
    self.acquire_node(0).1
  }

  #[inline(always)]
  pub fn set_root(&self, value: Ptr) {
    set_port(&mut self.acquire_node(0), P2, value)
  }

  #[inline(always)]
  pub fn compact(&self) -> Vec<AuxPorts> {
    let mut nodes = vec![];
    loop {
      let node = self.data[nodes.len()].lock().unwrap();
      let p1 = node.0;
      let p2 = node.1;
      if p1 != NULL || p2 != NULL {
        nodes.push((p1, p2));
      } else {
        break;
      }
    }
    return nodes;
  }
}


impl Net {
  // Creates an empty net with given size.
  pub fn new(size: usize) -> Self {
    Net {
      redexes: Default::default(),
      heap: Heap::new(size),
      anni: AtomicUsize::new(0),
      comm: AtomicUsize::new(0),
      eras: AtomicUsize::new(0),
      dref: AtomicUsize::new(0),
      oper: AtomicUsize::new(0),
    }
  }

  // Creates a net and boots from a REF.
  pub fn boot(&mut self, root_id: Val) {
    set_port(&mut self.heap.acquire_node(0), P2, Ptr::new(REF, root_id));
  }

  // Converts to a def.
  pub fn to_def(self) -> Def {
    Def { rdex: self.peek_current_redexes(), node: self.heap.compact() }
  }

  // Reads back from a def.
  pub fn from_def(def: Def) -> Self {
    let net = Net::new(def.node.len());
    for (i, &(p1, p2)) in def.node.iter().enumerate() {
      *net.heap.acquire_node(i as Val) = (p1, p2);
    }
    for r in def.rdex {
      net.push_redex(r);
    }
    net
  }

  // Gets a pointer's target.
  #[inline(always)]
  pub fn get_target(&self, ptr: Ptr) -> Ptr {
    let node = self.heap.acquire_node(ptr.val());
    if ptr.0 & 1 == P1 {
      node.0
    } else {
      node.1
    }
  }

  // // Sets a pointer's target.
  // #[inline(always)]
  // pub fn set_target(&self, ptr: Ptr, val: Ptr) {
  //   self.heap.set(ptr.val(), ptr.0 & 1, val)
  // }

  // Links two pointers, forming a new wire.
  pub fn link(&self, a: Ptr, b: Ptr) {
    // // Creates redex A-B
    // if a.is_pri() && b.is_pri() {
    //   if Ptr::can_skip(a, b) {
    //     self.eras.fetch_add(1, Ordering::Relaxed);
    //   } else {
    //     self.push_redex((a, b));
    //   }
    //   return;
    // }
    // // Substitutes A
    // if a.is_var() {
    //   self.set_target(a, b);
    // }
    // // Substitutes B
    // if b.is_var() {
    //   self.set_target(b, a);
    // }
    todo!()
  }

  // Links two pointers, forming a new wire.
  pub fn link_par(&self, a: Ptr, b: Ptr) {
    let (a_target, b_target) = loop {
      // ERA | REF | NUM
      let a_lock = if a.is_skp() { None } else { Some(self.heap.acquire_node(a.val())) };
      let b_lock = if b.is_skp() { None } else {
        let Some(b_lock) = self.heap.try_acquire_node(b.val()) else { continue };
        Some(b_lock)
      };
      break (a_lock, b_lock);
    };

    // Creates redex A-B
    if a.is_pri() && b.is_pri() {
      if Ptr::can_skip(a, b) {
        self.eras.fetch_add(1, Ordering::Relaxed);
      } else {
        self.push_redex((a, b));
      }
      return;
    }

    // Substitutes A
    if let Some(mut a_target) = a_target {
      match a.tag() {
        VR1 => a_target.0 = b,
        VR2 => a_target.1 = b,
        _ => {},
      }
    }

    if let Some(mut b_target) = b_target {
      match b.tag() {
        VR1 => b_target.0 = a,
        VR2 => b_target.1 = a,
        _ => {},
      }
    }
    // // Creates redex A-B
    // if a.is_pri() && b.is_pri() {
    //   if Ptr::can_skip(a, b) {
    //     self.eras.fetch_add(1, Ordering::Relaxed);
    //   } else {
    //     self.push_redex((a, b));
    //   }
    //   return;
    // }

    // match (a.is_var(), b.is_var()) {
    //   (false, false) => {}
    //   (true, false) => {
    //     self.set_target(a, b);
    //   }
    //   (false, true) => {
    //     self.set_target(b, a);
    //   }
    //   (true, true) => {
    //     // TODO: Use get_unchecked
    //     loop {
    //       let a_lock = self.heap.data[a.val() as usize].lock().unwrap();
    //       let Ok(mut b_lock) = self.heap.data[b.val() as usize].try_lock() else { continue };

    //       // Substitutes A
    //       if a.is_var() {
    //         let mut a_target_lock = self.heap.data[a.val() as usize].lock().unwrap();

    //         if a.0 & 1 == P1 {
    //           a_target_lock.0 = b;
    //         } else {
    //           a_target_lock.1 = b;
    //         }
    //       }
    //       // Substitutes B
    //       if b.is_var() {
    //         if b.0 & 1 == P1 {
    //           b_lock.0 = a;
    //         } else {
    //           b_lock.1 = a;
    //         }
    //       }
    //     }
    //   }
    // }
  }

  // Performs an interaction over a redex.
  pub fn interact(&self, book: &Book, a: Ptr, b: Ptr) {
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
      (REF, ERA)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
      (ERA, REF)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
      (ERA, ERA)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
      (VAR!(), _)      => self.link(a, b),
      (_, VAR!())      => self.link(b, a),
      (CTR!(), NUM)    => self.copy(a, b),
      (NUM, CTR!())    => self.copy(b, a),
      (NUM, ERA)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
      (ERA, NUM)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
      (NUM, NUM)       => { self.eras.fetch_add(1, Ordering::Relaxed); },
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

  // pub fn conn(&self, a: Ptr, b: Ptr) {
  //   self.anni.fetch_add(1, Ordering::Relaxed);
  //   self.link(self.heap.get(a.val(), P2), self.heap.get(b.val(), P2));
  //   self.heap.free(a.val());
  //   self.heap.free(b.val());
  // }

  pub fn anni(&self, a: Ptr, b: Ptr) {
    self.anni.fetch_add(1, Ordering::Relaxed);

    let (a0, a1) = self.heap.decompose(a.val());
    let (b0, b1) = self.heap.decompose(b.val());

    self.link_par(a0, b0);
    self.link_par(a1, b1);
  }

  pub fn comm(&self, a: Ptr, b: Ptr) {
    self.comm.fetch_add(1, Ordering::Relaxed);
    // We are getting allocated locations and their mutex guards
    // which prevents new threads from allocating that same region
    // while we hold this guard
    let ((mut m1, loc1), (mut m2, loc2), (mut m3, loc3), (mut m4, loc4)) = (self.heap.alloc(), self.heap.alloc(), self.heap.alloc(), self.heap.alloc());

    *m1 = (Ptr::new(VR1, loc3), Ptr::new(VR1, loc4));
    *m2 = (Ptr::new(VR2, loc3), Ptr::new(VR2, loc4));
    *m3 = (Ptr::new(VR1, loc1), Ptr::new(VR1, loc2));
    *m4 = (Ptr::new(VR2, loc1), Ptr::new(VR2, loc2));

    // We are dropping the mutex guards here, which allows
    // the linking part to proceed(also because it is not NULL anymore)
    drop(m1); drop(m2); drop(m3); drop(m4);

    // TODO: Use get_unchecked

    let (a0, a1) = self.heap.decompose(a.val());
    self.link_par(a0, Ptr::new(b.tag(), loc1));
    self.link_par(a1, Ptr::new(b.tag(), loc2));

    let (b0, b1) = self.heap.decompose(b.val());
    self.link_par(b0, Ptr::new(a.tag(), loc3));
    self.link_par(b1, Ptr::new(a.tag(), loc4));
  }

  pub fn pass(&self, a: Ptr, b: Ptr) {
    // self.comm.fetch_add(1, Ordering::Relaxed);

    // let ((m1, loc1), (m2, loc2), (m3, loc3)) = (self.heap.alloc(), self.heap.alloc(), self.heap.alloc());

    // let a_target = self.heap.acquire_node(a.val(), )

    // *m1 = (Ptr::new(VR2, loc2), Ptr::new(VR2, loc3));
    // *m2 = (self.heap.get(a.val(), P1), Ptr::new(VR1, loc1));
    // *m3 = (self.heap.get(a.val(), P1), Ptr::new(VR2, loc1));

    // self.link(self.heap.get(a.val(), P2), Ptr::new(b.tag(), loc1));
    // self.link(self.heap.get(b.val(), P1), Ptr::new(a.tag(), loc2));
    // self.link(self.heap.get(b.val(), P2), Ptr::new(a.tag(), loc3));
    // self.heap.free(a.val());
    // self.heap.free(b.val());
    todo!()
  }

  pub fn copy(&self, a: Ptr, b: Ptr) {
    // self.comm.fetch_add(1, Ordering::Relaxed);
    // self.link(self.heap.get(a.val(), P1), b);
    // self.link(self.heap.get(a.val(), P2), b);
    // self.heap.free(a.val());
    todo!()
  }

  pub fn era2(&self, a: Ptr) {
    // self.eras.fetch_add(1, Ordering::Relaxed);
    // self.link(self.heap.get(a.val(), P1), ERAS);
    // self.link(self.heap.get(a.val(), P2), ERAS);
    // self.heap.free(a.val());
    todo!()
  }

  pub fn era1(&self, a: Ptr) {
    // self.eras.fetch_add(1, Ordering::Relaxed);
    // self.link(self.heap.get(a.val(), P2), ERAS);
    // self.heap.free(a.val());
    todo!()
  }


  pub fn op2n(&self, a: Ptr, b: Ptr) {
    // self.oper.fetch_add(1, Ordering::Relaxed);
    // let mut p1 = self.heap.get(a.val(), P1);
    // // Optimization: perform chained ops at once
    // if p1.is_num() {
    //   let mut rt = b.val();
    //   let mut p2 = self.heap.get(a.val(), P2);
    //   loop {
    //     self.oper.fetch_add(1, Ordering::Relaxed);
    //     rt = self.prim(rt, p1.val());
    //     // If P2 is OP2, keep looping
    //     if p2.is_op2() {
    //       p1 = self.heap.get(p2.val(), P1);
    //       if p1.is_num() {
    //         p2 = self.heap.get(p2.val(), P2);
    //         self.oper.fetch_add(1, Ordering::Relaxed); // since OP1 is skipped
    //         continue;
    //       }
    //     }
    //     // If P2 is OP1, flip args and keep looping
    //     if p2.is_op1() {
    //       let tmp = rt;
    //       rt = self.heap.get(p2.val(), P1).val();
    //       p1 = Ptr::new(NUM, tmp);
    //       p2 = self.heap.get(p2.val(), P2);
    //       continue;
    //     }
    //     break;
    //   }
    //   self.link(Ptr::new(NUM, rt), p2);
    //   return;
    // }
    // self.heap.set(a.val(), P1, b);
    // self.link(Ptr::new(OP1, a.val()), p1);
    todo!()
  }

  pub fn op1n(&self, a: Ptr, b: Ptr) {
    // self.oper.fetch_add(1, Ordering::Relaxed);
    // let p1 = self.heap.get(a.val(), P1);
    // let p2 = self.heap.get(a.val(), P2);
    // let v0 = p1.val() as u32;
    // let v1 = b.val() as u32;
    // let v2 = self.prim(v0, v1);
    // self.link(Ptr::new(NUM, v2), p2);
    // self.heap.free(a.val());
    todo!()
  }

  pub fn prim(&self, a: u32, b: u32) -> u32 {
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

  pub fn mtch(&self, a: Ptr, b: Ptr) {
    // self.oper.fetch_add(1, Ordering::Relaxed);
    // let p1 = self.heap.get(a.val(), P1); // branch
    // let p2 = self.heap.get(a.val(), P2); // return
    // if b.val() == 0 {
    //   let loc = self.heap.alloc();
    //   self.heap.set(loc, P2, ERAS);
    //   self.link(p1, Ptr::new(CT0, loc));
    //   self.link(p2, Ptr::new(VR1, loc));
    //   self.heap.free(a.val());
    // } else {
    //   let (loc1, loc2) = (self.heap.alloc(), self.heap.alloc());
    //   self.heap.set(loc1, P1, ERAS);
    //   self.heap.set(loc1, P2, Ptr::new(CT0, loc2));
    //   self.heap.set(loc2, P1, Ptr::new(NUM, b.val() - 1));
    //   self.link(p1, Ptr::new(CT0, loc1));
    //   self.link(p2, Ptr::new(VR2, loc2));
    //   self.heap.free(a.val());
    // }
    todo!()
  }

  // Expands a closed net.
  #[inline(always)]
  pub fn deref(&self, book: &Book, ptr: Ptr, parent: Ptr) -> Ptr {
    self.dref.fetch_add(1, Ordering::Relaxed);
    let mut root = ptr;
    // FIXME: change "while" to "if" once lang prevents refs from returning refs
    while root.is_ref() {
      // Load the closed net.
      // TODO: Remove clone
      let mut def_net = unsafe { book.defs.get_unchecked((root.val() as usize) & 0xFFFFFF) }.clone();
      if def_net.node.len() == 0 {
        continue;
      }

      // TODO: Reuse Vec between calls (thread-local memory)
      let mut locs: Vec<(AcquiredNode, Val)> = Vec::with_capacity(def_net.node.len());
      locs.push((self.heap.acquire_node(parent.val()), parent.val()));
      locs.extend(
        def_net.node.iter()
          .skip(1) // skip root
          .map(|_| self.heap.alloc())
      );

      let adjust_ptr = #[inline(always)] |ptr: &mut Ptr| {
        if ptr.has_loc() {
          let (_, new_loc) = locs[ptr.val() as usize];
          *ptr = Ptr::new(ptr.tag(), new_loc);
        }
      };

      // Adjust all nodes
      (_, root) = def_net.node[0];
      adjust_ptr(&mut root);
      for (p1, p2) in &mut def_net.node[1..] {
        adjust_ptr(p1);
        adjust_ptr(p2);
      }

      for (a, b) in &def_net.rdex {
        let mut a = *a;
        let mut b = *b;
        adjust_ptr(&mut a);
        adjust_ptr(&mut b);
        self.push_redex((a, b));
      }

      // Load nodes and redexes
      for ((mut aux_ports, _new_loc), (p1, p2)) in locs.into_iter().skip(1).zip(&def_net.node[1..]) {
        *aux_ports = (*p1, *p2);
        // self.heap.set(loc, P1, *p1);
        // self.heap.set(loc, P2, *p2);
      }
    }
    return root;
  }

  // Reduces all redexes.
  pub fn reduce(&self, book: &Book) {
    // let mut redex_snapshot = std::mem::take(&mut self.redexes);
    // while redex_snapshot.len() > 0 {
    //   for (a, b) in redex_snapshot.drain(..) {
    //     self.interact(book, a, b);
    //   }
    //   std::mem::swap(&mut self.redexes, &mut redex_snapshot);
    // }

    while !self.redexes.is_empty() {
      rayon::scope(|s| {
        while let Some((a, b)) = self.pop_redex() {
          s.spawn(move |_| {
            self.interact(book, a, b);
          });
        }
      });
    }
  }

  // Reduce a net to normal form.
  pub fn normal(&self, book: &Book) {
    // expand
    // while queue not empty:
    //   while queue not empty:
    //     scope
    //       for all r in queue: interact and push to queue
    //   expand

    self.expand(book, ROOT);
    while !self.redexes.is_empty() {
      self.reduce(book);
      self.expand(book, ROOT);
    }
  }

  // Expands heads.
  pub fn expand(&self, book: &Book, dir: Ptr) {
    let ptr = self.get_target(dir);
    if ptr.is_ctr() {
      self.expand(book, Ptr::new(VR1, ptr.val()));
      self.expand(book, Ptr::new(VR2, ptr.val()));
    } else if ptr.is_ref() {
      let _exp = self.deref(book, ptr, dir);
      // self.set_target(dir, exp);
    }
  }

  // Total rewrite count.
  pub fn rewrites(&self) -> usize {
    self.anni_() + self.comm_() + self.eras() + self.dref() + self.oper()
  }

  pub(crate) fn pop_redex(&self) -> Option<Redex> {
    self.redexes.pop()
  }

  /// NOTE: Must be called while no other thread is pushing/popping redexes
  pub fn peek_current_redexes(&self) -> Vec<Redex> {
    let mut r = Vec::with_capacity(self.redexes.len());
    while let Some(redex) = self.pop_redex() {
      r.push(redex);
    }
    for redex in r.iter() {
      self.push_redex(*redex);
    }
    r
  }

  pub(crate) fn push_redex(&self, r: Redex) {
    // self.tx_redex_next_frame.send(r).unwrap();
    self.redexes.push(r);
  }
}
