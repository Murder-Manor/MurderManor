use std::collections::{
    HashMap,
    HashSet,
};
use std::error;

use rand::seq::SliceRandom;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Object {
    pub uuid: Uuid,
    pub taken_by: HashSet<Uuid>,
}

#[derive(Default)]
pub struct Objects {
    pub objects: HashMap<Uuid, Object>,
    pub takable_objects: Vec<Uuid>,
}

impl Objects {
    pub fn take_random_takable_object(&self) -> Uuid {
        self.takable_objects.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn takers_for(&self, object_id: Uuid) -> HashSet<Uuid> {
        match self.objects.get(&object_id) {
            Some(object) => object.taken_by.clone(),
            None => return HashSet::<Uuid>::new(),
        }
    }

    // Take an object
    pub fn take_object(&mut self, object_id: Uuid, player_id: Uuid) -> Result<(), Box<dyn error::Error>> {
        let object = match self.objects.get_mut(&object_id) {
            Some(object) => object,
            None => {
                let object = Object {
                    uuid: object_id,
                    taken_by: HashSet::<Uuid>::new(),
                };
                self.objects.insert(object_id, object);
                self.objects.get_mut(&object_id).unwrap()
            }
        };

        object.taken_by.insert(player_id);
        Ok(())
    }

    pub fn reset(&mut self) {
        self.objects = HashMap::<Uuid, Object>::new();
    }
}
