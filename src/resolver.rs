
use std::hash::{hash, Hash, Hasher, SipHasher};

#[derive(Debug)]
pub struct GroupsConfig {
    groups: Vec<(String, u64)>,
    weight_sum: u64
}

impl GroupsConfig {
    pub fn new() -> GroupsConfig {
        GroupsConfig { groups: Vec::new(), weight_sum: 0 }
    }

    pub fn add(&mut self, name: String, weight: u64) {
        self.groups.push((name, weight));
        self.weight_sum += weight;
    }
}

pub struct GroupResolver {
    config: GroupsConfig
}

impl GroupResolver {
    pub fn new(config: GroupsConfig ) -> GroupResolver {
        GroupResolver{ config: config }
    }

    pub fn resolve_group(&self, customer: String) -> String {
        let hash = stable_hash(&customer);
        let bucket = hash % self.config.weight_sum;

        let mut current_bucket = 0;
        let mut group = "default".to_string();
        for &(ref name, ref weight) in self.config.groups.iter() {
            current_bucket += *weight;
            if bucket < current_bucket {
                group = name.clone();
                break;
            }
        }

        return group;
    }
}

fn stable_hash(value: &String) -> u64 {
    let mut hasher = SipHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
