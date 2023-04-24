use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

use glam::IVec3;

use crate::models::{Entity, IBounds3};

#[derive(Debug)]
pub enum EntityCmd {
    Add(Entity),
    Remove(String),
}

pub struct EntityClient {
    group_bounds: IBounds3,
    cmds: Vec<EntityCmd>,
}

impl EntityClient {
    pub fn new(group_bounds: IBounds3, cmds: Vec<EntityCmd>) -> Self {
        Self { group_bounds, cmds }
    }
}

const GROUP_SIZE: f32 = 16.0;

pub struct EntityService {
    entities: HashMap<String, Entity>,
    group_index: HashMap<IVec3, HashSet<String>>,
    clients: HashMap<String, EntityClient>,
}

impl EntityService {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            group_index: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        if self.entities.contains_key(&entity.id) {
            panic!("entity is already existed at id {:?}", entity.id);
        }

        self.entities.insert(entity.id.clone(), entity.clone());

        let group = entity.pos.div(GROUP_SIZE).floor().as_ivec3();
        if !self.group_index.contains_key(&group) {
            self.group_index.insert(group, HashSet::new());
        }
        self.group_index
            .get_mut(&group)
            .unwrap()
            .insert(entity.id.clone());

        for (_, client) in &mut self.clients {
            if client.group_bounds.inclusive_contains(&group) {
                client.cmds.push(EntityCmd::Add(entity.clone()));
            }
        }
    }

    pub fn remove_entity(&mut self, id: String) {
        if !self.entities.contains_key(&id) {
            panic!("entity is not found at id {:?}", id);
        }

        let entity = self.entities.remove(&id).unwrap();

        let group = entity.pos.div(GROUP_SIZE).floor().as_ivec3();
        self.group_index.get_mut(&group).unwrap().remove(&entity.id);
        if self.group_index.get(&group).unwrap().is_empty() {
            self.group_index.remove(&group);
        }

        for (_, client) in &mut self.clients {
            if client.group_bounds.inclusive_contains(&group) {
                client.cmds.push(EntityCmd::Remove(id.clone()));
            }
        }
    }

    pub fn get_entity(&self, id: String) -> Option<Entity> {
        self.entities.get(&id).cloned()
    }

    pub fn set_bounds(&mut self, client_name: String, bounds: IBounds3) {
        let min = bounds.min.as_vec3a().div(GROUP_SIZE).floor().as_ivec3();
        let max = bounds.max.as_vec3a().div(GROUP_SIZE).floor().as_ivec3();
        let group_bounds = IBounds3::new(min, max);

        if let Some(client) = self.clients.get_mut(&client_name) {
            if client.group_bounds != group_bounds {
                for x in client.group_bounds.min.x..=client.group_bounds.max.x {
                    for y in client.group_bounds.min.y..=client.group_bounds.max.y {
                        for z in client.group_bounds.min.z..=client.group_bounds.max.z {
                            let group = IVec3::new(x, y, z);
                            if !group_bounds.inclusive_contains(&group) {
                                if let Some(ids) = self.group_index.get(&group) {
                                    for id in ids {
                                        client.cmds.push(EntityCmd::Remove(id.clone()));
                                    }
                                }
                            }
                        }
                    }
                }

                for x in group_bounds.min.x..=group_bounds.max.x {
                    for y in group_bounds.min.y..=group_bounds.max.y {
                        for z in group_bounds.min.z..=group_bounds.max.z {
                            let group = IVec3::new(x, y, z);
                            if !client.group_bounds.inclusive_contains(&group) {
                                if let Some(ids) = self.group_index.get(&group) {
                                    for id in ids {
                                        let entity = self.entities.get(id).unwrap();
                                        client.cmds.push(EntityCmd::Add(entity.clone()));
                                    }
                                }
                            }
                        }
                    }
                }

                client.group_bounds = group_bounds;
            }
        } else {
            let mut cmds = vec![];

            for x in group_bounds.min.x..=group_bounds.max.x {
                for y in group_bounds.min.y..=group_bounds.max.y {
                    for z in group_bounds.min.z..=group_bounds.max.z {
                        let group = IVec3::new(x, y, z);
                        if let Some(ids) = self.group_index.get(&group) {
                            for id in ids {
                                let entity = self.entities.get(id).unwrap();
                                cmds.push(EntityCmd::Add(entity.clone()));
                            }
                        }
                    }
                }
            }

            let client = EntityClient::new(group_bounds, cmds);
            self.clients.insert(client_name, client);
        }
    }

    pub fn get_cmds(&mut self, client_name: String) -> Vec<EntityCmd> {
        let Some(client) = self.clients.get_mut(&client_name) else {
            panic!("client named {:?} is not found", client_name);
        };

        let mut out_cmds = vec![];
        std::mem::swap(&mut client.cmds, &mut out_cmds);
        out_cmds
    }
}

#[cfg(test)]
mod tests {
    use glam::{IVec3, Vec3A};

    use crate::models::{Entity, IBounds3};

    use super::{EntityCmd, EntityService};

    #[test]
    fn add_tile() {
        let mut service = EntityService::new();
        service.add_entity(Entity::new(
            "TEST_ENTITY_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));

        let entity = service.get_entity("TEST_ENTITY_ID".to_string()).unwrap();
        assert_eq!(entity.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(entity.resource_name, "TEST_RESOURCE_NAME");
    }

    #[test]
    fn remove_tile() {
        let mut service = EntityService::new();
        service.add_entity(Entity::new(
            "TEST_ENTITY_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.remove_entity("TEST_ENTITY_ID".to_string());

        let is_none = service.get_entity("TEST_ENTITY_ID".to_string()).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = EntityService::new();
        service.set_bounds(
            "TEST_CLIENT_NAME".to_string(),
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        service.add_entity(Entity::new(
            "TEST_ENTITY_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_entity(Entity::new(
            "TEST_OTHER_ENTITY_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        let cmds = service.get_cmds("TEST_CLIENT_NAME".to_string());
        let [EntityCmd::Add(entity)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(entity.id, "TEST_ENTITY_ID".to_string());
        assert_eq!(entity.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(entity.resource_name, "TEST_RESOURCE_NAME".to_string());
    }

    #[test]
    fn set_bounds_after_fill_data() {
        let mut service = EntityService::new();
        service.add_entity(Entity::new(
            "TEST_ENTITY_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_entity(Entity::new(
            "TEST_OTHER_ENTITY_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        service.set_bounds(
            "TEST_CLIENT_NAME".to_string(),
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        let cmds = service.get_cmds("TEST_CLIENT_NAME".to_string());
        let [EntityCmd::Add(entity)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(entity.id, "TEST_ENTITY_ID".to_string());
        assert_eq!(entity.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(entity.resource_name, "TEST_RESOURCE_NAME".to_string());
    }
}
