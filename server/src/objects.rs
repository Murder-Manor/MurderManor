use std::collections::HashMap;
use std::error;

use rand::seq::SliceRandom;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Object {
    pub uuid: Uuid,
    pub taken_by: Vec<Uuid>,
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

    pub fn takers_for(&self, object_id: Uuid) -> Vec<Uuid> {
        match self.objects.get(&object_id) {
            Some(object) => object.taken_by.clone(),
            None => return vec![],
        }
    }

    pub fn is_takable(&self, object_id: &Uuid) -> bool {
        self.takable_objects.contains(object_id)
    }

    // Take an object and return the position of the player in the takers
    pub fn take_object(&mut self, object_id: Uuid, player_id: Uuid) -> Result<usize, Box<dyn error::Error>> {
        let object = match self.objects.get_mut(&object_id) {
            Some(object) => object,
            None => {
                let object = Object {
                    uuid: object_id,
                    taken_by: vec![],
                };
                self.objects.insert(object_id, object);
                self.objects.get_mut(&object_id).unwrap()
            }
        };

        object.taken_by.push(player_id);
        Ok(object.taken_by.len() - 1)
    }

    pub fn get_object_takers(&self, object_id: Uuid) -> Option<Vec<Uuid>> {
        match self.objects.get(&object_id) {
            Some(object) => Some(object.taken_by.clone()),
            None => None,
        }
    }
}
