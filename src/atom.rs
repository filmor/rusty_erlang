use std::collections::{HashMap};
use std::sync::atomic::{AtomicUint, Ordering};
use std::rc::{Rc, Weak};

use types::{Uint};
use term;

#[allow(dead_code)]
pub struct AtomTable {
  // atomic counter for atom index
  index:    AtomicUint,
  // maps atom name to Eterm
  entries:  HashMap<String, Rc<term::Eterm>>,

  pub am_start: Rc<term::Eterm>,
}
pub type Table = AtomTable;

impl AtomTable {
  pub fn new() -> AtomTable {
    let mut a = AtomTable{
      index:    AtomicUint::new(0),
      entries:  HashMap::new(),
      am_start: Rc::new(term::Eterm::Nil),
    };
    a.am_start = a.put(&"start".to_string());
    return a
  }

  // Adds an atom to atom table. Returns Rc'd atom Eterm
  pub fn put(&mut self, name: &String) -> Rc<term::Eterm> {
    match self.entries.get(name) {
      Some(x) => return x.clone(), // users don't own atoms, just weakrefs
      None    => ()
    }
    let index: uint     = self.index.fetch_add(1, Ordering::SeqCst);
    let at = Rc::new(term::Eterm::Atom(term::make_atom(index)));
    self.entries.insert(name.clone(), at.clone());
    return at;
  }
}
