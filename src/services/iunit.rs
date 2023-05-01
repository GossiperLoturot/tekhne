use crate::models::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum IUnitCmd {
    Add(IUnit),
    Remove(IVec3),
}

#[derive(Debug)]
pub struct IUnitClient {
    id: String,
    bounds: IBounds3,
    cmds: Vec<IUnitCmd>,
}

impl IUnitClient {
    pub fn new(id: String, bounds: IBounds3, cmds: Vec<IUnitCmd>) -> Self {
        Self { id, bounds, cmds }
    }
}

#[derive(Debug, Default, Resource)]
pub struct IUnitService {
    iunits: HashMap<IVec3, IUnit>,
    clients: HashMap<String, IUnitClient>,
}

impl IUnitService {
    pub fn add_iunit(&mut self, iunit: IUnit) {
        if self.iunits.contains_key(&iunit.pos) {
            panic!("iunit is already existed at pos {:?}.", iunit.pos);
        }

        self.iunits.insert(iunit.pos, iunit.clone());

        for (_, client) in &mut self.clients {
            if client.bounds.inclusive_contains(&iunit.pos) {
                client.cmds.push(IUnitCmd::Add(iunit.clone()));
            }
        }
    }

    pub fn remove_iunit(&mut self, pos: IVec3) {
        if !self.iunits.contains_key(&pos) {
            panic!("iunit is not found at pos {:?}", pos);
        }

        self.iunits.remove(&pos);

        for (_, client) in &mut self.clients {
            if client.bounds.inclusive_contains(&pos) {
                client.cmds.push(IUnitCmd::Remove(pos));
            }
        }
    }

    pub fn get_iunit(&self, pos: IVec3) -> Option<IUnit> {
        self.iunits.get(&pos).cloned()
    }

    pub fn set_bounds(&mut self, client_name: &str, bounds: IBounds3) {
        if let Some(client) = self.clients.get_mut(client_name) {
            if client.bounds != bounds {
                for x in client.bounds.min.x..=client.bounds.max.x {
                    for y in client.bounds.min.y..=client.bounds.max.y {
                        for z in client.bounds.min.z..=client.bounds.max.z {
                            let pos = IVec3::new(x, y, z);
                            if !bounds.inclusive_contains(&pos) && self.iunits.contains_key(&pos) {
                                client.cmds.push(IUnitCmd::Remove(pos));
                            }
                        }
                    }
                }

                for x in bounds.min.x..=bounds.max.x {
                    for y in bounds.min.y..=bounds.max.y {
                        for z in bounds.min.z..=bounds.max.z {
                            let pos = IVec3::new(x, y, z);
                            if !client.bounds.inclusive_contains(&pos)
                                && self.iunits.contains_key(&pos)
                            {
                                let iunit = self.iunits.get(&pos).unwrap();
                                client.cmds.push(IUnitCmd::Add(iunit.clone()));
                            }
                        }
                    }
                }

                client.bounds = bounds;
            }
        } else {
            let mut cmds = vec![];

            for x in bounds.min.x..=bounds.max.x {
                for y in bounds.min.y..=bounds.max.y {
                    for z in bounds.min.z..=bounds.max.z {
                        let pos = IVec3::new(x, y, z);
                        if self.iunits.contains_key(&pos) {
                            let iunit = self.iunits.get(&pos).unwrap();
                            cmds.push(IUnitCmd::Add(iunit.clone()));
                        }
                    }
                }
            }

            let client = IUnitClient::new(client_name.to_string(), bounds, cmds);
            self.clients.insert(client_name.to_string(), client);
        }
    }

    pub fn get_cmds(&mut self, client_name: &str) -> Vec<IUnitCmd> {
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
    fn add_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));

        let iunit = service.get_iunit(IVec3::new(0, 0, 0)).unwrap();
        assert_eq!(iunit.pos, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_name, "TEST_RESOURCE_NAME");
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.remove_iunit(IVec3::new(0, 0, 0));

        let is_none = service.get_iunit(IVec3::new(0, 0, 0)).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = IUnitService::default();
        service.set_bounds(
            "TEST_CLIENT_NAME",
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_iunit(IUnit::new(
            IVec3::new(-1, -1, -1),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        let cmds = service.get_cmds("TEST_CLIENT_NAME");
        let [IUnitCmd::Add(iunit)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(iunit.pos, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }

    #[test]
    fn set_bounds_after_fill_data() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_iunit(IUnit::new(
            IVec3::new(-1, -1, -1),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        service.set_bounds(
            "TEST_CLIENT_NAME",
            IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)),
        );

        let cmds = service.get_cmds("TEST_CLIENT_NAME");
        let [IUnitCmd::Add(iunit)] = &cmds[..] else {
            panic!("unexpected cmds {:?}", cmds);
        };
        assert_eq!(iunit.pos, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }
}
