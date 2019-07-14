use crate::scene::{Hitable, HitRecord, HitableID};
use std::collections::HashMap;

pub struct HitableLibrary {
     library: HashMap<HitableID, Box<Hitable + Send>>,
     id_count: HitableID
}

impl HitableLibrary {
    pub fn new() -> HitableLibrary {
        HitableLibrary {
            library: HashMap::new(),
            id_count: 0
        }
    }

    pub fn add_hitable_to_library(&mut self, mut hitable: Box<Hitable + Send>) -> HitableID {
        self.id_count += 1;
        hitable.set_hitable_id(self.id_count);
        self.library.insert(self.id_count, hitable);
        return self.id_count;
    }

    pub fn checkout_hitable(&self, id: HitableID) -> Option<&Box<Hitable + Send>> {
        return self.library.get(&id);
    }
}