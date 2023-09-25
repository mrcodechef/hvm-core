use crate::core::{*};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Heap {
  data: Vec<u32>,
  next: usize,
  used: usize,
}

impl Heap {
  pub fn new(size: usize) -> Heap {
    return Heap {
      data: vec![0; size * 2],
      next: 1,
      used: 0,
    };
  }

  #[inline(always)]
  pub fn alloc(&mut self, size: usize) -> Val {
    if size == 0 {
      return 0;
    } else {
      let mut space = 0;
      loop {
        if self.next >= self.data.len() {
          space = 0;
          self.next = 1;
        }
        if self.get(self.next as Val, P1).is_nil() {
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
    self.used -= 1;
    *self.at_mut(index, P1) = NULL;
    *self.at_mut(index, P2) = NULL;
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
  pub fn at(&self, index: Val, port: Port) -> &Ptr {
    unsafe {
      let val : &u32 = self.data.get_unchecked((index * 2 + port) as usize);
      let ptr : &Ptr = std::mem::transmute(val);
      return ptr;
    }
  }

  #[inline(always)]
  pub fn at_mut(&mut self, index: Val, port: Port) -> &mut Ptr {
    unsafe {
      let val : &mut u32 = self.data.get_unchecked_mut((index * 2 + port) as usize);
      let ptr : &mut Ptr = std::mem::transmute(val);
      return ptr;
    }
  }

  #[inline(always)]
  pub fn get(&self, index: Val, port: Port) -> Ptr {
    return *self.at(index, port);
  }

  #[inline(always)]
  pub fn set(&mut self, index: Val, port: Port, value: Ptr) {
    *self.at_mut(index, port) = value;
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
  pub fn compact(&mut self) -> (Ptr, Vec<(Ptr,Ptr)>) {
    let root = self.data[1];
    let mut node = vec![];
    for i in 0 .. self.used {
      node.push((Ptr(self.data[(i+1)*2+0]), Ptr(self.data[(i+1)*2+1])));
    }
    return (Ptr(root), node);
  }
}
