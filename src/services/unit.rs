use crate::models::*;
use glam::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum UnitCmd {
    Add(Unit),
    Remove(String),
}

#[derive(Debug)]
pub struct UnitClient {
    id: String,
    group_bounds: IBounds3,
    cmds: Vec<UnitCmd>,
}

impl UnitClient {
    pub fn new(id: String, group_bounds: IBounds3, cmds: Vec<UnitCmd>) -> Self {
        Self {
            id,
            group_bounds,
            cmds,
        }
    }
}

const GROUP_SIZE: f32 = 16.0;

#[derive(Debug, Default)]
pub struct UnitService {
    units: HashMap<String, Unit>,
    group_index: HashMap<IVec3, HashSet<String>>,
    clients: HashMap<String, UnitClient>,
}

impl UnitService {
    pub fn add_unit(&mut self, unit: Unit) {
        if self.units.contains_key(&unit.id) {
            panic!("unit is already existed at id {:?}", unit.id);
        }

        self.units.insert(unit.id.clone(), unit.clone());

        let group = (unit.pos / GROUP_SIZE).floor().as_ivec3();
        if !self.group_index.contains_key(&group) {
            self.group_index.insert(group, HashSet::new());
        }
        self.group_index
            .get_mut(&group)
            .unwrap()
            .insert(unit.id.clone());

        for (_, client) in &mut self.clients {
            if client.group_bounds.inclusive_contains(&group) {
                client.cmds.push(UnitCmd::Add(unit.clone()));
            }
        }
    }

    pub fn remove_unit(&mut self, id: &str) {
        if !self.units.contains_key(id) {
            panic!("unit is not found at id {:?}", id);
        }

        let unit = self.units.remove(id).unwrap();

        let group = (unit.pos / GROUP_SIZE).floor().as_ivec3();
        self.group_index.get_mut(&group).unwrap().remove(&unit.id);
        if self.group_index.get(&group).unwrap().is_empty() {
            self.group_index.remove(&group);
        }

        for (_, client) in &mut self.clients {
            if client.group_bounds.inclusive_contains(&group) {
                client.cmds.push(UnitCmd::Remove(id.to_string()));
            }
        }
    }

    pub fn get_unit(&self, id: &str) -> Option<Unit> {
        self.units.get(id).cloned()
    }

    pub fn set_bounds(&mut self, client_name: &str, bounds: IBounds3) {
        let min = (bounds.min.as_vec3() / GROUP_SIZE).floor().as_ivec3();
        let max = (bounds.max.as_vec3() / GROUP_SIZE).floor().as_ivec3();
        let group_bounds = IBounds3::new(min, max);

        if let Some(client) = self.clients.get_mut(client_name) {
            if client.group_bounds != group_bounds {
                for x in client.group_bounds.min.x..=client.group_bounds.max.x {
                    for y in client.group_bounds.min.y..=client.group_bounds.max.y {
                        for z in client.group_bounds.min.z..=client.group_bounds.max.z {
                            let group = IVec3::new(x, y, z);
                            if !group_bounds.inclusive_contains(&group) {
                                if let Some(ids) = self.group_index.get(&group) {
                                    for id in ids {
                                        client.cmds.push(UnitCmd::Remove(id.clone()));
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
                                        let unit = self.units.get(id).unwrap();
                                        client.cmds.push(UnitCmd::Add(unit.clone()));
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
                                let unit = self.units.get(id).unwrap();
                                cmds.push(UnitCmd::Add(unit.clone()));
                            }
                        }
                    }
                }
            }

            let client = UnitClient::new(client_name.to_string(), group_bounds, cmds);
            self.clients.insert(client_name.to_string(), client);
        }
    }

    pub fn get_cmds(&mut self, client_name: &str) -> Vec<UnitCmd> {
        let Some(client) = self.clients.get_mut(client_name) else {
            panic!("client named {:?} is not found", client_name);
        };

        let mut out_cmds = vec![];
        std::mem::swap(&mut client.cmds, &mut out_cmds);
        out_cmds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));

        let unit = service.get_unit("TEST_UNIT_ID").unwrap();
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_name, "TEST_RESOURCE_NAME");
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.remove_unit("TEST_UNIT_ID");

        let is_none = service.get_unit("TEST_UNIT_ID").is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = UnitService::default();
        service.set_bounds(
            "TEST_CLIENT_NAME",
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_unit(Unit::new(
            "TEST_OTHER_UNIT_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        let cmds = service.get_cmds("TEST_CLIENT_NAME");
        let [UnitCmd::Add(unit)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(unit.id, "TEST_UNIT_ID".to_string());
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }

    #[test]
    fn set_bounds_after_fill_data() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_unit(Unit::new(
            "TEST_OTHER_UNIT_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        service.set_bounds(
            "TEST_CLIENT_NAME",
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        let cmds = service.get_cmds("TEST_CLIENT_NAME");
        let [UnitCmd::Add(unit)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(unit.id, "TEST_UNIT_ID".to_string());
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }
}
