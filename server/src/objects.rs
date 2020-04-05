use std::collections::HashMap;
use std::error;

use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Object {
    pub uuid: Uuid,
    pub taken_by: Vec<Uuid>,
}

#[derive(Default)]
pub struct Objects {
    pub objects: HashMap<Uuid, Object>,
}

impl Objects {
    pub fn take_object(&mut self, object_id: Uuid, player_id: Uuid) -> Result<(), Box<dyn error::Error>> {
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
        Ok(())
    }

    pub fn get_object_takers(&self, object_id: Uuid) -> Option<Vec<Uuid>> {
        match self.objects.get(&object_id) {
            Some(object) => Some(object.taken_by.clone()),
            None => None,
        }
    }
}
