// An efficient Interaction Combinator runtime
// ===========================================
// This file implements an efficient interaction combinator runtime. Nodes are represented by 2 aux
// ports (P1, P2), with the main port (P1) omitted. A separate vector, 'rdex', holds main ports,
// and, thus, tracks active pairs that can be reduced in parallel. Pointers are unboxed, meaning
// that Ptr::ERAs, NUMs and REFs don't use any additional space. REFs lazily expand to closed nets when
// they interact with nodes, and are cleared when they interact with Ptr::ERAs, allowing for constant
// space evaluation of recursive functions on Scott encoded datatypes.

use crate::{
  jit::{Instruction, Trg},
  ops::Op,
  trace,
  trace::Tracer,
};
use std::{
  alloc::{self, Layout},
  fmt,
  hint::unreachable_unchecked,
  sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    Arc, Barrier,
  },
  thread, ptr::NonNull, num::NonZeroUsize,
};

// -------------------
//   Primitive Types
// -------------------

pub type Lab = u16;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tag {
  Red = 0,
  Var = 1,
  Ref = 2,
  Num = 3,
  Op2 = 4,
  Op1 = 5,
  Mat = 6,
  Ctr = 7,
}

use Tag::*;

impl TryFrom<u8> for Tag {
  type Error = ();

  #[inline(always)]
  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => Tag::Red,
      1 => Tag::Var,
      2 => Tag::Ref,
      3 => Tag::Num,
      4 => Tag::Op2,
      5 => Tag::Op1,
      6 => Tag::Mat,
      7 => Tag::Ctr,
      _ => Err(())?,
    })
  }
}

/// A tagged pointer.
#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Default)]
#[repr(transparent)]
#[must_use]
pub struct Port(pub u64);

impl fmt::Debug for Port {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:016x?} ", self.0)?;
    match *self {
      Port::ERA => write!(f, "[ERA]"),
      Port::FREE => write!(f, "[FREE]"),
      Port::GONE => write!(f, "[GONE]"),
      Port::LOCK => write!(f, "[LOCK]"),
      _ => match self.tag() {
        Num => write!(f, "[Num {}]", self.num()),
        Var | Red | Ref | Mat => write!(f, "[{:?} {:?}]", self.tag(), self.loc()),
        Op2 | Op1 | Ctr => write!(f, "[{:?} {:?} {:?}]", self.tag(), self.lab(), self.loc()),
      },
    }
  }
}

impl Port {
  pub const ERA: Port = Port(Ref as _);
  pub const FREE: Port = Port(0x8000_0000_0000_0000);
  pub const LOCK: Port = Port(0xFFFF_FFFF_FFFF_FFF0);
  pub const GONE: Port = Port(0xFFFF_FFFF_FFFF_FFFF);

  #[inline(always)]
  pub fn new(tag: Tag, lab: Lab, loc: Loc) -> Self {
    Port(((lab as u64) << 48) | (loc.0.get() as u64) | (tag as u64))
  }

  #[inline(always)]
  pub fn new_var(loc: Loc) -> Self {
    Port::new(Var, 0, loc)
  }

  #[inline(always)]
  pub const fn new_num(val: u64) -> Self {
    Port((val << 4) | (Num as u64))
  }

  #[inline(always)]
  pub fn new_ref(def: &Def) -> Port {
    unsafe { Port::new(Ref, def.lab, Loc::from_num(def as *const _ as _)) }
  }

  #[inline(always)]
  pub fn tag(&self) -> Tag {
    unsafe { ((self.0 & 0x7) as u8).try_into().unwrap_unchecked() }
  }

  #[inline(always)]
  pub const fn lab(&self) -> Lab {
    (self.0 >> 48) as Lab
  }

  #[inline(always)]
  pub fn op(&self) -> Op {
    unsafe { self.lab().try_into().unwrap_unchecked() }
  }

  #[inline(always)]
  pub const fn loc(&self) -> Loc {
    unsafe { Loc::from_num((self.0 & 0x0000_FFFF_FFFF_FFF8) as usize) }
  }

  #[inline(always)]
  pub const fn num(&self) -> u64 {
    self.0 >> 4
  }

  #[inline(always)]
  pub fn wire(&self) -> Wire {
    Wire::new(self.loc())
  }

  #[inline(always)]
  pub fn is_principal(&self) -> bool {
    self.tag() >= Ref
  }

  #[inline(always)]
  pub fn is_skippable(&self) -> bool {
    matches!(self.tag(), Num | Ref)
  }

  #[inline(always)]
  pub fn is_ctr(&self, lab: Lab) -> bool {
    self.tag() == Ctr && self.lab() == lab
  }

  #[inline(always)]
  pub fn redirect(&self) -> Port {
    Port::new(Red, 0, self.loc())
  }

  #[inline(always)]
  pub fn unredirect(&self) -> Port {
    Port::new(Var, 0, self.loc())
  }
}

pub struct TraverseNode {
  pub lab: Lab,
  pub p1: Wire,
  pub p2: Wire,
}

pub struct TraverseOp1 {
  pub op: Op,
  pub num: Port,
  pub p2: Wire,
}

impl Port {
  #[inline(always)]
  pub fn consume_node(self) -> TraverseNode {
    self.traverse_node()
  }

  #[inline(always)]
  pub fn traverse_node(self) -> TraverseNode {
    TraverseNode { lab: self.lab(), p1: Wire::new(self.loc()), p2: Wire::new(self.loc().other_half()) }
  }

  #[inline(always)]
  pub fn consume_op1(self) -> TraverseOp1 {
    let op = self.op();
    let s = self.consume_node();
    let num = s.p1.swap_target(Port::FREE);
    TraverseOp1 { op, num, p2: s.p2 }
  }

  #[inline(always)]
  pub fn traverse_op1(self) -> TraverseOp1 {
    let op = self.op();
    let s = self.traverse_node();
    let num = s.p1.load_target();
    TraverseOp1 { op, num, p2: s.p2 }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Half {
  Left,
  Right,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[must_use]
pub struct Loc(NonZeroUsize);

impl fmt::Debug for Loc {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:012x?}", self.0)
  }
}

impl Loc {
  const HALF_MASK: usize = 0b1000;

  pub unsafe fn from_ptr(ptr: *const AtomicU64) -> Self {
    Self(NonZeroUsize::new(ptr as usize).unwrap_unchecked())
  }
  pub const unsafe fn from_num(ptr: usize) -> Self {
    // Unwrap, but const
    Self(match NonZeroUsize::new(ptr) {
      Some(x) => x,
      None => unreachable_unchecked(),
    })
  }
  #[inline(always)]
  pub fn as_ptr(&self) -> *const AtomicU64 {
    self.0.get() as *const _
  }
  #[inline(always)]
  pub fn val<'a>(&self) -> &'a AtomicU64 {
    unsafe { self.as_ptr().as_ref().unwrap_unchecked() }
  }

  #[inline(always)]
  pub fn left_half(&self) -> Self {
    unsafe { Loc::from_num(self.0.get() & !Loc::HALF_MASK) }
  }

  #[inline(always)]
  pub fn other_half(&self) -> Self {
    unsafe { Loc::from_num(self.0.get() ^ Loc::HALF_MASK) }
  }

  #[inline(always)]
  pub fn def<'a>(&self) -> &'a Def {
    unsafe { &*(self.0.get() as *const _) }
  }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[must_use]
pub struct Wire(NonNull<AtomicU64>);

impl fmt::Debug for Wire {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:012x?}", self.0.as_ptr() as usize)
  }
}

unsafe impl Send for Wire {}

impl Wire {
  pub unsafe fn from_ptr(ptr: *const AtomicU64) -> Self {
    Self(NonNull::new(ptr as _).unwrap_unchecked())
  }
  #[inline(always)]
  pub fn as_ptr(&self) -> *const AtomicU64 {
    self.0.as_ptr()
  }

  #[inline(always)]
  pub fn loc(&self) -> Loc {
    unsafe { Loc(NonZeroUsize::new(self.0.as_ptr() as usize).unwrap_unchecked()) }
  }

  #[inline(always)]
  pub fn new(loc: Loc) -> Wire {
    unsafe { Wire::from_ptr(loc.0.get() as _) }
  }

  #[inline(always)]
  fn target<'a>(&self) -> &'a AtomicU64 {
    unsafe { self.0.as_ptr().as_ref().unwrap_unchecked() }
  }

  #[inline(always)]
  pub fn load_target(&self) -> Port {
    Port(self.target().load(Relaxed))
  }

  #[inline(always)]
  pub fn set_target(&self, port: Port) {
    self.target().store(port.0, Relaxed);
  }

  #[inline(always)]
  pub fn cas_target(&self, expected: Port, value: Port) -> Result<Port, Port> {
    self.target().compare_exchange_weak(expected.0, value.0, Relaxed, Relaxed).map(Port).map_err(Port)
  }

  #[inline(always)]
  pub fn swap_target(&self, value: Port) -> Port {
    Port(self.target().swap(value.0, Relaxed))
  }

  // Takes a pointer's target.
  #[inline(always)]
  pub fn lock_target(&self) -> Port {
    loop {
      let got = self.swap_target(Port::LOCK);
      if got != Port::LOCK {
        return got;
      }
    }
  }
}

#[derive(Clone, Debug)]
#[repr(align(16))]
pub struct Def {
  pub lab: Lab,
  pub inner: DefType,
}

#[derive(Clone, Debug)]
pub enum DefType {
  Native(fn(&mut Net, Port)),
  Net(DefNet),
}

/// A compact closed net, used for dereferences.
#[derive(Clone, Debug, Default)]
pub struct DefNet {
  pub instr: Vec<Instruction>,
}

// -----------
//   The Net
// -----------

#[repr(C)]
#[repr(align(16))]
#[derive(Default)]
pub struct Node(pub AtomicU64, pub AtomicU64);

// The global node buffer.
pub type Area = [Node];

/// Rewrite counter.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rewrites {
  pub anni: usize, // anni rewrites
  pub comm: usize, // comm rewrites
  pub eras: usize, // eras rewrites
  pub dref: usize, // dref rewrites
  pub oper: usize, // oper rewrites
}

impl Rewrites {
  pub fn add_to(&self, target: &AtomicRewrites) {
    target.anni.fetch_add(self.anni, Relaxed);
    target.comm.fetch_add(self.comm, Relaxed);
    target.eras.fetch_add(self.eras, Relaxed);
    target.dref.fetch_add(self.dref, Relaxed);
    target.oper.fetch_add(self.oper, Relaxed);
  }

  // Total rewrite count.
  pub fn total(&self) -> usize {
    self.anni + self.comm + self.eras + self.dref + self.oper
  }
}

/// Rewrite counter, atomic.
#[derive(Default)]
pub struct AtomicRewrites {
  pub anni: AtomicUsize, // anni rewrites
  pub comm: AtomicUsize, // comm rewrites
  pub eras: AtomicUsize, // eras rewrites
  pub dref: AtomicUsize, // dref rewrites
  pub oper: AtomicUsize, // oper rewrites
}

impl AtomicRewrites {
  pub fn add_to(&self, target: &mut Rewrites) {
    target.anni += self.anni.load(Relaxed);
    target.comm += self.comm.load(Relaxed);
    target.eras += self.eras.load(Relaxed);
    target.dref += self.dref.load(Relaxed);
    target.oper += self.oper.load(Relaxed);
  }
}

// A interaction combinator net.
pub struct Net<'a> {
  pub tid: usize,              // thread id
  pub tids: usize,             // thread count
  pub rdex: Vec<(Port, Port)>, // redexes
  pub trgs: Vec<Trg>,
  pub rwts: Rewrites, // rewrite count
  pub quik: Rewrites, // quick rewrite count
  pub root: Option<Wire>,
  // allocator
  pub area: &'a Area,
  pub head: Option<Loc>,
  pub next: usize,
  //
  pub tracer: Tracer,
}

impl<'a> Net<'a> {
  // Creates an empty net with a given heap.
  pub fn new(area: &'a Area) -> Self {
    let mut net = Net::new_with_root(area, None);
    net.root = Some(Wire::new(net.alloc()));
    net
  }

  // Creates an empty net with a given heap.
  pub fn new_with_root(area: &'a Area, root: Option<Wire>) -> Self {
    Net {
      tid: 0,
      tids: 1,
      rdex: vec![],
      trgs: vec![Trg::Port(Port::FREE); 1 << 16],
      rwts: Rewrites::default(),
      quik: Rewrites::default(),
      root,
      area,
      head: None,
      next: 0,
      tracer: Tracer::default(),
    }
  }

  // Boots a net from a Ref.
  pub fn boot(&mut self, def: &Def) {
    let def = Port::new_ref(def);
    trace!(self.tracer, def);
    self.root.as_ref().unwrap().set_target(def);
  }
}

// -------------
//   Allocator
// -------------

impl<'a> Net<'a> {
  pub fn init_heap(size: usize) -> Box<[Node]> {
    unsafe {
      Box::from_raw(core::ptr::slice_from_raw_parts_mut(
        alloc::alloc(Layout::array::<Node>(size).unwrap()) as *mut _,
        size,
      ))
    }
  }

  pub fn head_or_zero(&self) -> usize {
    self.head.as_ref().map(|x| x.0.get()).unwrap_or(0)
  }

  #[inline(never)]
  pub fn half_free(&mut self, loc: Loc) {
    const FREE: u64 = Port::FREE.0;
    trace!(self.tracer, loc);
    loc.val().store(FREE, Relaxed);
    if loc.other_half().val().load(Relaxed) == FREE {
      trace!(self.tracer, "other free");
      let loc = loc.left_half();
      if loc.val().compare_exchange(FREE, self.head_or_zero() as u64, Relaxed, Relaxed).is_ok() {
        let old_head = &self.head;
        let new_head = old_head.as_ref().map(|_| loc);
        trace!(self.tracer, "appended", old_head, new_head);
        self.head = new_head;
      } else {
        trace!(self.tracer, "too slow");
      };
    }
  }

  #[inline(never)]
  pub fn alloc(&mut self) -> Loc {
    trace!(self.tracer, self.head);
    let loc = if let Some(head) = &self.head {
      let loc = head.clone();
      let next = unsafe { Loc::from_num(head.val().load(Relaxed) as usize) };
      trace!(self.tracer, next);
      self.head = Some(next);
      loc
    } else {
      let index = self.next;
      self.next += 1;
      // Note: Here we cast to *const Node instead of *const Port
      // because we want to be able to use all of the node
      unsafe { Loc::from_num(self.area.get(index).expect("OOM") as *const Node as usize) }
    };
    trace!(self.tracer, loc, self.head);
    loc.val().store(Port::LOCK.0, Relaxed);
    loc.other_half().val().store(Port::LOCK.0, Relaxed);
    loc.other_half()
  }
}

pub struct CreatedNode {
  pub p0: Port,
  pub p1: Port,
  pub p2: Port,
}

impl<'a> Net<'a> {
  #[inline(always)]
  pub fn create_node(&mut self, tag: Tag, lab: Lab) -> CreatedNode {
    let loc = self.alloc();
    CreatedNode {
      p0: Port::new(tag, lab, loc.clone()),
      p1: Port::new_var(loc.clone()),
      p2: Port::new_var(loc.other_half()),
    }
  }

  #[inline(always)]
  pub fn create_wire(&mut self, port: Port) -> Wire {
    let loc = self.alloc();
    self.half_free(loc.other_half());
    let wire = Wire::new(loc);
    self.link_port_port(port, Port::new_var(wire.loc()));
    wire
  }
}

// ----------
//   Linker
// ----------

impl<'a> Net<'a> {
  // Links two pointers, forming a new wire. Assumes ownership.
  #[inline(always)]
  pub fn link_port_port(&mut self, a_port: Port, b_port: Port) {
    trace!(self.tracer, a_port, b_port);
    if a_port.is_principal() && b_port.is_principal() {
      self.redux(a_port, b_port);
    } else {
      self.half_link_port_port(a_port.clone(), b_port.clone());
      self.half_link_port_port(b_port, a_port);
    }
  }

  // Given two locations, links both stored pointers, atomically.
  #[inline(always)]
  pub fn link_wire_wire(&mut self, a_wire: Wire, b_wire: Wire) {
    trace!(self.tracer, a_wire, b_wire);
    let a_port = a_wire.lock_target();
    let b_port = b_wire.lock_target();
    trace!(self.tracer, a_port, b_port);
    if a_port.is_principal() && b_port.is_principal() {
      self.half_free(a_wire.loc());
      self.half_free(b_wire.loc());
      self.redux(a_port, b_port);
    } else {
      self.half_link_wire_port(a_port.clone(), a_wire, b_port.clone());
      self.half_link_wire_port(b_port, b_wire, a_port);
    }
  }

  // Given a location, link the pointer stored to another pointer, atomically.
  #[inline(always)]
  pub fn link_wire_port(&mut self, a_wire: Wire, b_port: Port) {
    trace!(self.tracer, a_wire, b_port);
    let a_port = a_wire.lock_target();
    trace!(self.tracer, a_port);
    if a_port.is_principal() && b_port.is_principal() {
      self.half_free(a_wire.loc());
      self.redux(a_port, b_port);
    } else {
      self.half_link_wire_port(a_port.clone(), a_wire, b_port.clone());
      self.half_link_port_port(b_port, a_port);
    }
  }

  #[inline(always)]
  pub fn redux(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    if a.is_skippable() && b.is_skippable() {
      self.rwts.eras += 1;
    } else {
      self.rdex.push((a, b));
    }
  }

  // When two threads interfere, uses the lock-free link algorithm described on the 'paper/'.
  #[inline(always)]
  pub fn half_link_port_port(&mut self, a_port: Port, b_port: Port) {
    trace!(self.tracer, a_port, b_port);
    if a_port.tag() == Var {
      a_port.wire().set_target(b_port);
    }
  }

  // When two threads interfere, uses the lock-free link algorithm described on the 'paper/'.
  #[inline(always)]
  pub fn half_link_wire_port(&mut self, a_port: Port, a_wire: Wire, b_port: Port) {
    trace!(self.tracer, a_port, a_wire, b_port);
    // If 'a_port' is a var...
    if a_port.tag() == Var {
      let got = a_port.wire().cas_target(Port::new_var(a_wire.loc()), b_port.clone());
      // Attempts to link using a compare-and-swap.
      if got.is_ok() {
        trace!(self.tracer, "cas ok");
        self.half_free(a_wire.loc());
      // If the CAS failed, resolve by using redirections.
      } else {
        trace!(self.tracer, "cas fail", got.clone().unwrap_err());
        if b_port.tag() == Var {
          let port = b_port.redirect();
          a_wire.set_target(port);
          //self.atomic_linker_var(a_port, a_wire, b_port);
        } else if b_port.is_principal() {
          a_wire.set_target(b_port.clone());
          self.atomic_linker_pri(a_port, a_wire, b_port);
        } else {
          unreachable!();
        }
      }
    } else {
      self.half_free(a_wire.loc());
    }
  }

  // Atomic linker for when 'b_port' is a principal port.
  pub fn atomic_linker_pri(&mut self, mut a_port: Port, a_wire: Wire, b_port: Port) {
    trace!(self.tracer);
    loop {
      trace!(self.tracer, a_port, a_wire, b_port);
      // Peek the target, which may not be owned by us.
      let mut t_wire = a_port.wire();
      let mut t_port = t_wire.load_target();
      trace!(self.tracer, t_port);
      // If it is taken, we wait.
      if t_port == Port::LOCK {
        continue;
      }
      // If target is a rewireection, we own it. Clear and move forward.
      if t_port.tag() == Red {
        self.half_free(t_wire.loc());
        a_port = t_port;
        continue;
      }
      // If target is a variable, we don't own it. Try replacing it.
      if t_port.tag() == Var {
        if t_wire.cas_target(t_port.clone(), b_port.clone()).is_ok() {
          trace!(self.tracer, "var cas ok");
          // Clear source location.
          self.half_free(a_wire.loc());
          // Collect the orphaned backward path.
          t_wire = t_port.wire();
          t_port = t_wire.load_target();
          while t_port.tag() == Red {
            trace!(self.tracer, t_wire, t_port);
            self.half_free(t_wire.loc());
            t_wire = t_port.wire();
            t_port = t_wire.load_target();
          }
          return;
        }
        trace!(self.tracer, "var cas fail");
        // If the CAS failed, the var changed, so we try again.
        continue;
      }

      // If it is a node, two threads will reach this branch.
      if t_port.is_principal() || t_port == Port::GONE {
        // Sort references, to avoid deadlocks.
        let x_wire = if a_wire < t_wire { a_wire.clone() } else { t_wire.clone() };
        let y_wire = if a_wire < t_wire { t_wire.clone() } else { a_wire.clone() };
        trace!(self.tracer, x_wire, y_wire);
        // Swap first reference by Ptr::GONE placeholder.
        let x_port = x_wire.swap_target(Port::GONE);
        // First to arrive creates a redex.
        if x_port != Port::GONE {
          let y_port = y_wire.swap_target(Port::GONE);
          trace!(self.tracer, "fst", x_wire, y_wire, x_port, y_port);
          self.redux(x_port, y_port);
          return;
        // Second to arrive clears up the memory.
        } else {
          trace!(self.tracer, "snd", x_wire, y_wire);
          self.half_free(x_wire.loc());
          while y_wire.cas_target(Port::GONE, Port::LOCK).is_err() {}
          self.half_free(y_wire.loc());
          return;
        }
      }
      // Shouldn't be reached.
      trace!(self.tracer, t_port, a_wire, a_port, b_port);
      unreachable!()
    }
  }

  // Atomic linker for when 'b_port' is an aux port.
  pub fn atomic_linker_var(&mut self, _: Port, _: Wire, b_port: Port) {
    loop {
      let ste_wire = b_port.clone().wire();
      let ste_port = ste_wire.load_target();
      if ste_port.tag() == Var {
        let trg_wire = ste_port.wire();
        let trg_port = trg_wire.load_target();
        if trg_port.tag() == Red {
          let neo_port = trg_port.unredirect();
          if ste_wire.cas_target(ste_port, neo_port).is_ok() {
            self.half_free(trg_wire.loc());
            continue;
          }
        }
      }
      break;
    }
  }
}

// ---------------------
//   Interaction Rules
// ---------------------

impl<'a> Net<'a> {
  // Performs an interaction over a redex.
  #[inline(always)]
  pub fn interact(&mut self, a: Port, b: Port) {
    self.tracer.sync();
    trace!(self.tracer, a, b);
    match (a.tag(), b.tag()) {
      // not actually an active pair
      (Var | Red, _) | (_, Var | Red) => unreachable!(),
      // nil-nil
      (Num | Ref, Num | Ref) => self.rwts.eras += 1,
      // comm 2/2
      (Ctr, Mat) if a.lab() != 0 => self.comm22(a, b),
      (Mat, Ctr) if b.lab() != 0 => self.comm22(a, b),
      (Ctr, Op2) | (Op2, Ctr) => self.comm22(a, b),
      (Ctr, Ctr) if a.lab() != b.lab() => self.comm22(a, b),
      // comm 1/2
      (Op1, Ctr) => self.comm12(a, b),
      (Ctr, Op1) => self.comm12(b, a),
      // anni
      (Mat, Mat) | (Op2, Op2) | (Ctr, Ctr) => self.anni2(a, b),
      (Op1, Op1) => self.anni1(a, b),
      // comm 2/0
      (Ref, Ctr) if b.lab() >= a.lab() => self.comm02(a, b),
      (Ctr, Ref) if a.lab() >= b.lab() => self.comm02(b, a),
      (Num, Ctr) => self.comm02(a, b),
      (Ctr, Num) => self.comm02(b, a),
      (Ref, _) if a == Port::ERA => self.comm02(a, b),
      (_, Ref) if b == Port::ERA => self.comm02(b, a),
      // deref
      (Ref, _) => self.call(a, b),
      (_, Ref) => self.call(b, a),
      // native ops
      (Op2, Num) => self.op2_num(a, b),
      (Num, Op2) => self.op2_num(b, a),
      (Op1, Num) => self.op1_num(a, b),
      (Num, Op1) => self.op1_num(b, a),
      (Mat, Num) => self.mat_num(a, b),
      (Num, Mat) => self.mat_num(b, a),
      // todo: what should the semantics of these be?
      (Mat, Ctr) // b.tag() == 0
      | (Ctr, Mat) // a.tag() == 0
      | (Op2, Op1)
      | (Op1, Op2)
      | (Op2, Mat)
      | (Mat, Op2)
      | (Op1, Mat)
      | (Mat, Op1) => todo!(),
    }
  }

  #[inline(never)]
  /// ```text
  ///
  ///         a2 |   | a1
  ///           _|___|_
  ///           \     /
  ///         a  \   /
  ///             \ /
  ///              |
  ///             / \
  ///         b  /   \
  ///           /_____\
  ///            |   |
  ///         b1 |   | b2
  ///
  /// --------------------------- anni2
  ///
  ///         a2 |   | a1
  ///            |   |
  ///             \ /
  ///              X
  ///             / \
  ///            |   |
  ///         b1 |   | b2
  ///
  /// ```
  pub fn anni2(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.anni += 1;
    let a = a.consume_node();
    let b = b.consume_node();
    self.link_wire_wire(a.p1, b.p1);
    self.link_wire_wire(a.p2, b.p2);
  }

  #[inline(never)]
  /// ```text
  ///
  ///         a2 |   | a1
  ///           _|___|_
  ///           \     /
  ///         a  \   /
  ///             \ /
  ///              |
  ///             /#\
  ///         b  /###\
  ///           /#####\
  ///            |   |
  ///         b1 |   | b2
  ///
  /// --------------------------- comm22
  ///
  ///     a2 |         | a1
  ///        |         |
  ///       /#\       /#\
  ///  B2  /###\     /###\  B1
  ///     /#####\   /#####\
  ///      |   \     /   |
  ///   p1 | p2 \   / p1 | p2
  ///      |     \ /     |
  ///      |      X      |
  ///      |     / \     |
  ///   p2 | p1 /   \ p2 | p1
  ///     _|___/_   _\___|_
  ///     \     /   \     /
  ///  A1  \   /     \   /  A2
  ///       \ /       \ /
  ///        |         |
  ///     b1 |         | b2
  ///
  /// ```
  pub fn comm22(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.comm += 1;

    let a = a.consume_node();
    let b = b.consume_node();

    let A1 = self.create_node(Ctr, a.lab);
    let A2 = self.create_node(Ctr, a.lab);
    let B1 = self.create_node(Ctr, b.lab);
    let B2 = self.create_node(Ctr, b.lab);

    trace!(self.tracer, A1.p0, A2.p0, B1.p0, B2.p0);
    self.link_port_port(A1.p1, B1.p1);
    self.link_port_port(A1.p2, B2.p1);
    self.link_port_port(A2.p1, B1.p2);
    self.link_port_port(A2.p2, B2.p2);

    trace!(self.tracer);
    self.link_wire_port(a.p1, B1.p0);
    self.link_wire_port(a.p2, B2.p0);
    self.link_wire_port(b.p1, A1.p0);
    self.link_wire_port(b.p2, A2.p0);
  }

  #[inline(never)]
  /// ```text
  ///
  ///         a  (---)
  ///              |
  ///              |
  ///             /#\
  ///         b  /###\
  ///           /#####\
  ///            |   |
  ///         b1 |   | b2
  ///
  /// --------------------------- comm02
  ///
  ///     a (---)   (---) a
  ///         |       |
  ///      b1 |       | b2
  ///
  /// ```
  pub fn comm02(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.comm += 1;
    let b = b.consume_node();
    self.link_wire_port(b.p1, a.clone());
    self.link_wire_port(b.p2, a);
  }

  #[inline(never)]
  /// ```text
  ///
  ///         a2 |
  ///            |   n
  ///           _|___|_
  ///           \     /
  ///         a  \op1/
  ///             \ /
  ///              |
  ///             / \
  ///         b  /op1\
  ///           /_____\
  ///            |   |
  ///            m   |
  ///                | b2
  ///
  /// --------------------------- anni1
  ///
  ///         a2 |
  ///            |
  ///            |
  ///             \
  ///              \
  ///               \
  ///                |
  ///                |
  ///                | b2
  ///
  /// ```
  pub fn anni1(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.anni += 1;
    let a = a.consume_op1();
    let b = b.consume_op1();
    self.link_wire_wire(a.p2, b.p2);
  }

  /// ```text
  ///
  ///         a2 |   n
  ///           _|___|_
  ///           \     /
  ///         a  \op1/
  ///             \ /
  ///              |
  ///             /#\
  ///         b  /###\
  ///           /#####\
  ///            |   |
  ///         b1 |   | b2
  ///
  /// --------------------------- comm12
  ///
  ///     a2 |
  ///        |
  ///       /#\
  ///  B2  /###\
  ///     /#####\
  ///      |   \
  ///   p1 | p2 \
  ///      |     \
  ///      |      \
  ///      |       \
  ///   p2 |   n    \ p2 n
  ///     _|___|_   _\___|_
  ///     \     /   \     /
  ///  A1  \op1/     \op1/  A2
  ///       \ /       \ /
  ///        |         |
  ///     b1 |         | b2
  ///
  /// ```
  #[inline(never)]
  pub fn comm12(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.comm += 1;

    let a = a.consume_op1();
    let b = b.consume_node();

    let A1 = self.create_node(Ctr, a.op as Lab);
    let A2 = self.create_node(Ctr, a.op as Lab);
    let B2 = self.create_node(Ctr, b.lab);

    trace!(self.tracer, B2.p0, A1.p0, A2.p0);
    self.link_port_port(A1.p1, a.num.clone());
    self.link_port_port(A1.p2, B2.p1);
    self.link_port_port(A2.p1, a.num.clone());
    self.link_port_port(A2.p2, B2.p2);

    trace!(self.tracer);
    self.link_wire_port(a.p2, B2.p0);
    self.link_wire_port(b.p1, A1.p0);
    self.link_wire_port(b.p2, A2.p0);
  }

  #[inline(never)]
  /// ```text
  ///
  ///         a  (---)
  ///              |
  ///              |
  ///             / \
  ///         b  /op1\
  ///           /_____\
  ///            |   |
  ///            n   |
  ///                | b2
  ///
  /// --------------------------- comm02
  ///
  ///              (---) a
  ///                |
  ///                | b2
  ///
  /// ```
  pub fn comm01(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.comm += 1;
    let b = b.consume_op1();
    self.link_wire_port(b.p2, a);
  }

  #[inline(never)]
  /// ```text
  ///                             |
  ///         b   (0)             |         b  (n+1)
  ///              |              |              |
  ///              |              |              |
  ///             / \             |             / \
  ///         a  /mat\            |         a  /mat\
  ///           /_____\           |           /_____\
  ///            |   |            |            |   |
  ///         a1 |   | a2         |         a1 |   | a2
  ///                             |
  /// --------------------------- | -----------X--------------- mat_num
  ///                             |          _ _ _ _ _
  ///                             |        /           \
  ///                             |    y2 |  (n) y1     |
  ///                             |      _|___|_        |
  ///                             |      \     /        |
  ///               _             |    y  \   /         |
  ///             /   \           |        \ /          |
  ///    x2 (*)  | x1  |          |      x2 |  (*) x1   |
  ///       _|___|_    |          |        _|___|_      |
  ///       \     /    |          |        \     /      |
  ///     x  \   /     |          |      x  \   /       |
  ///         \ /      |          |          \ /        |
  ///          |       |          |           |         |
  ///       a1 |       | a2       |        a1 |         | a2
  ///                             |
  /// ```
  pub fn mat_num(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.oper += 1;
    let a = a.consume_node();
    let b = b.num();
    if b == 0 {
      let x = self.create_node(Ctr, 0);
      trace!(self.tracer, x.p0);
      self.link_port_port(x.p2, Port::ERA);
      self.link_wire_port(a.p2, x.p1);
      self.link_wire_port(a.p1, x.p0);
    } else {
      let x = self.create_node(Ctr, 0);
      let y = self.create_node(Ctr, 0);
      trace!(self.tracer, x.p0, y.p0);
      self.link_port_port(x.p1, Port::ERA);
      self.link_port_port(x.p2, y.p0);
      self.link_port_port(y.p1, Port::new_num(b - 1));
      self.link_wire_port(a.p2, y.p2);
      self.link_wire_port(a.p1, x.p0);
    }
  }

  #[inline(never)]
  /// ```text
  ///                   
  ///         b   (n)    
  ///              |      
  ///              |       
  ///             / \       
  ///         a  /op2\       
  ///           /_____\       
  ///            |   |         
  ///         a1 |   | a2       
  ///                            
  /// --------------------------- op2_num
  ///           _ _ _
  ///         /       \
  ///        |   n     |   
  ///       _|___|_    |   
  ///       \     /    |   
  ///     x  \op1/     |   
  ///         \ /      |   
  ///          |       |   
  ///       a1 |       | a2  
  ///                       
  /// ```
  pub fn op2_num(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.oper += 1;
    let a = a.consume_node();
    let x = self.create_node(Op1, a.lab);
    trace!(self.tracer, x.p0);
    self.link_port_port(x.p1, b);
    self.link_wire_port(a.p2, x.p2);
    self.link_wire_port(a.p1, x.p0);
  }

  #[inline(never)]
  /// ```text
  ///                   
  ///         b   (m)    
  ///              |      
  ///              |       
  ///             / \       
  ///         a  /op1\       
  ///           /_____\       
  ///            |   |         
  ///            n   |         
  ///                | a2       
  ///                            
  /// --------------------------- op2_num
  ///                       
  ///          (n opr m)
  ///              |         
  ///              | a2
  ///                       
  /// ```
  pub fn op1_num(&mut self, a: Port, b: Port) {
    trace!(self.tracer, a, b);
    self.rwts.oper += 1;
    let a = a.consume_op1();
    let n = a.num.num();
    let m = b.num();
    let out = a.op.op(n, m);
    self.link_wire_port(a.p2, Port::new_num(out));
  }

  // Expands a closed net.
  #[inline(never)]
  pub fn call(&mut self, port: Port, trg: Port) {
    trace!(self.tracer, port, trg);
    self.rwts.dref += 1;
    // Intercepts with a native function, if available.
    let def = port.loc().def();
    let net = match &def.inner {
      DefType::Native(native) => return native(self, trg),
      DefType::Net(net) => net,
    };

    self.set_trg(0, Trg::Port(trg));
    for i in &net.instr {
      unsafe {
        match *i {
          Instruction::Const(ref port, trg) => self.set_trg(trg, Trg::Port(port.clone())),
          Instruction::Link(a, b) => self.link_trg(self.get_trg(a), self.get_trg(b)),
          Instruction::Set(t, ref p) => {
            if !p.is_principal() {
              unreachable_unchecked()
            }
            self.link_trg_port(self.get_trg(t), p.clone())
          }
          Instruction::Ctr(lab, t, a, b) => {
            let (at, bt) = self.do_ctr(self.get_trg(t), lab);
            self.set_trg(a, at);
            self.set_trg(b, bt);
          }
          Instruction::Op2(op, t, a, b) => {
            let (at, bt) = self.do_op2(self.trgs[t].clone(), op);
            self.set_trg(a, at);
            self.set_trg(b, bt);
          }
          Instruction::Op1(op, n, t, b) => {
            let bt = self.do_op1(self.trgs[t].clone(), op, n);
            self.set_trg(b, bt);
          }
          Instruction::Mat(t, a, b) => {
            let (at, bt) = self.do_mat(self.trgs[t].clone());
            self.set_trg(a, at);
            self.set_trg(b, bt);
          }
        }
      }
    }
  }

  #[inline(always)]
  fn get_trg(&self, i: usize) -> Trg {
    unsafe { self.trgs.get_unchecked(i).clone() }
  }

  #[inline(always)]
  fn set_trg(&mut self, i: usize, trg: Trg) {
    unsafe { *self.trgs.get_unchecked_mut(i) = trg }
  }
}

impl<'a> Net<'a> {
  pub fn rewrites(&self) -> Rewrites {
    self.rwts
  }
}

// -----------------
//   Normalization
// -----------------

impl<'a> Net<'a> {
  // Reduces all redexes.
  #[inline(always)]
  pub fn reduce(&mut self, limit: usize) -> usize {
    let mut count = 0;
    while let Some((a, b)) = self.rdex.pop() {
      self.interact(a, b);
      count += 1;
      if count >= limit {
        break;
      }
    }
    count
  }

  // Expands heads.
  #[inline(always)]
  pub fn expand(&mut self) {
    fn go(net: &mut Net, wire: Wire, len: usize, key: usize) {
      trace!(net.tracer, wire);
      let port = wire.load_target();
      trace!(net.tracer, port);
      if port == Port::LOCK {
        return;
      }
      if port.tag() == Ctr {
        let node = port.traverse_node();
        if len >= net.tids || key % 2 == 0 {
          go(net, node.p1, len * 2, key / 2);
        }
        if len >= net.tids || key % 2 == 1 {
          go(net, node.p2, len * 2, key / 2);
        }
      } else if port.tag() == Ref && port != Port::ERA {
        let got = wire.swap_target(Port::LOCK);
        if got != Port::LOCK {
          trace!(net.tracer, port, wire);
          net.call(port, Port::new_var(wire.loc()));
        }
      }
    }
    go(self, self.root.clone().unwrap(), 1, self.tid);
  }

  // Reduce a net to normal form.
  pub fn normal(&mut self) {
    self.expand();
    while !self.rdex.is_empty() {
      self.reduce(usize::MAX);
      self.expand();
    }
  }

  // Forks into child threads, returning a Net for the (tid/tids)'th thread.
  fn fork(&self, tid: usize, tids: usize) -> Self {
    let heap_size = self.area.len() / tids;
    let heap_start = heap_size * tid;
    let area = &self.area[heap_start .. heap_start + heap_size];
    let mut net = Net::new_with_root(area, self.root.clone());
    net.next = self.next.saturating_sub(heap_start);
    net.head = if tid == 0 { self.head.clone() } else { None };
    net.tid = tid;
    net.tids = tids;
    net.tracer.set_tid(tid);
    let from = self.rdex.len() * (tid + 0) / tids;
    let upto = self.rdex.len() * (tid + 1) / tids;
    for i in from .. upto {
      net.rdex.push(self.rdex[i].clone());
    }
    net
  }

  // Evaluates a term to normal form in parallel
  pub fn parallel_normal(&mut self) {
    const SHARE_LIMIT: usize = 1 << 12; // max share redexes per split
    const LOCAL_LIMIT: usize = 1 << 18; // max local rewrites per epoch

    // Local thread context
    struct ThreadContext<'a> {
      tid: usize,                             // thread id
      tlog2: usize,                           // log2 of thread count
      tick: usize,                            // current tick
      net: Net<'a>,                           // thread's own net object
      delta: &'a AtomicRewrites,              // global delta rewrites
      quick: &'a AtomicRewrites,              // global delta rewrites
      share: &'a Vec<(AtomicU64, AtomicU64)>, // global share buffer
      rlens: &'a Vec<AtomicUsize>,            // global redex lengths
      total: &'a AtomicUsize,                 // total redex length
      barry: Arc<Barrier>,                    // synchronization barrier
    }

    // Initialize global objects
    let cores = std::thread::available_parallelism().unwrap().get() as usize;
    let tlog2 = cores.ilog2() as usize;
    let tids = 1 << tlog2;
    let delta = AtomicRewrites::default(); // delta rewrite counter
    let quick = AtomicRewrites::default(); // quick rewrite counter
    let rlens = (0 .. tids).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();
    let share = (0 .. SHARE_LIMIT * tids).map(|_| Default::default()).collect::<Vec<_>>();
    let total = AtomicUsize::new(0); // sum of redex bag length
    let barry = Arc::new(Barrier::new(tids)); // global barrier

    // Perform parallel reductions
    std::thread::scope(|s| {
      for tid in 0 .. tids {
        let mut ctx = ThreadContext {
          tid,
          tick: 0,
          net: self.fork(tid, tids),
          tlog2,
          delta: &delta,
          quick: &quick,
          share: &share,
          rlens: &rlens,
          total: &total,
          barry: Arc::clone(&barry),
        };
        thread::Builder::new().name(format!("t{:02x?}", ctx.net.tid)).spawn_scoped(s, move || main(&mut ctx)).unwrap();
      }
    });

    // Clear redexes and sum stats
    self.rdex.clear();
    delta.add_to(&mut self.rwts);
    quick.add_to(&mut self.quik);

    // Main reduction loop
    #[inline(always)]
    fn main(ctx: &mut ThreadContext) {
      loop {
        reduce(ctx);
        ctx.net.expand();
        if count(ctx) == 0 {
          break;
        }
      }
      ctx.net.rwts.add_to(ctx.delta);
      ctx.net.quik.add_to(ctx.quick);
    }

    // Reduce redexes locally, then share with target
    #[inline(always)]
    fn reduce(ctx: &mut ThreadContext) {
      loop {
        ctx.net.reduce(LOCAL_LIMIT);
        if count(ctx) == 0 {
          break;
        }
        let tlog2 = ctx.tlog2;
        split(ctx, tlog2);
        ctx.tick += 1;
      }
    }

    // Count total redexes (and populate 'rlens')
    #[inline(always)]
    fn count(ctx: &mut ThreadContext) -> usize {
      ctx.barry.wait();
      ctx.total.store(0, Relaxed);
      ctx.barry.wait();
      ctx.rlens[ctx.tid].store(ctx.net.rdex.len(), Relaxed);
      ctx.total.fetch_add(ctx.net.rdex.len(), Relaxed);
      ctx.barry.wait();
      ctx.total.load(Relaxed)
    }

    // Share redexes with target thread
    #[inline(always)]
    fn split(ctx: &mut ThreadContext, plog2: usize) {
      unsafe {
        let side = (ctx.tid >> (plog2 - 1 - (ctx.tick % plog2))) & 1;
        let shift = (1 << (plog2 - 1)) >> (ctx.tick % plog2);
        let a_tid = ctx.tid;
        let b_tid = if side == 1 { a_tid - shift } else { a_tid + shift };
        let a_len = ctx.net.rdex.len();
        let b_len = ctx.rlens[b_tid].load(Relaxed);
        let send = if a_len > b_len { (a_len - b_len) / 2 } else { 0 };
        let recv = if b_len > a_len { (b_len - a_len) / 2 } else { 0 };
        let send = std::cmp::min(send, SHARE_LIMIT);
        let recv = std::cmp::min(recv, SHARE_LIMIT);
        for i in 0 .. send {
          let init = a_len - send * 2;
          let rdx0 = ctx.net.rdex.get_unchecked(init + i * 2 + 0).clone();
          let rdx1 = ctx.net.rdex.get_unchecked(init + i * 2 + 1).clone();
          //let init = 0;
          //let ref0 = ctx.net.rdex.get_unchecked_mut(init + i * 2 + 0);
          //let rdx0 = *ref0;
          //*ref0    = (Ptr(0), Ptr(0));
          //let ref1 = ctx.net.rdex.get_unchecked_mut(init + i * 2 + 1);
          //let rdx1 = *ref1;
          //*ref1    = (Ptr(0), Ptr(0));
          let targ = ctx.share.get_unchecked(b_tid * SHARE_LIMIT + i);
          *ctx.net.rdex.get_unchecked_mut(init + i) = rdx0;
          targ.0.store(rdx1.0.0, Relaxed);
          targ.1.store(rdx1.1.0, Relaxed);
        }
        ctx.net.rdex.truncate(a_len - send);
        ctx.barry.wait();
        for i in 0 .. recv {
          let got = ctx.share.get_unchecked(a_tid * SHARE_LIMIT + i);
          ctx.net.rdex.push((Port(got.0.load(Relaxed)), Port(got.1.load(Relaxed))));
        }
      }
    }
  }
}
