extern crate core;


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::disk_pers::{create_or_load_map_env, MapEnv};

    fn clear_files(name: &str) {
        if fs::remove_file(name).is_ok() {};
    }

    #[test]
    fn create_env_and_check_name() {
        let map_env: MapEnv = create_or_load_map_env("name1");
        assert_eq!(map_env.get_name(), "name1");
        clear_files("name1");
    }

    #[test]
    fn create_env_put_and_get() {
        let mut map_env: MapEnv = create_or_load_map_env("name2");
        map_env.put("somekey".to_string(), "somevalue".to_string());
        let value = map_env.get("somekey".to_string()).unwrap();
        assert_eq!(value, "somevalue");
        clear_files("name2");
    }

    #[test]
    fn create_env_put_and_delete() {
        let mut map_env: MapEnv = create_or_load_map_env("name4");
        map_env.put("somekey".to_string(), "somevalue".to_string());
        map_env.delete("somekey".to_string());
        let value = map_env.get("somekey".to_string());
        assert_eq!(value, None);
        clear_files("name4");
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

pub struct PersistenceEnv {
    name: String,
}

impl PersistenceEnv {
    pub fn save_to_local(&self, to_save: &str) {

        let mut file = File::create(&self.name).unwrap();
        file.write_all(to_save.as_bytes()).expect("could not write");
    }

    pub fn load_from_local(&self) -> String {
        let mut file = File::open(&self.name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("could not read");
        contents
    }

    pub fn init_env(&self, with: &str) {
        if File::open(&self.name).is_ok() {} else {
            let mut file = File::create(&self.name).unwrap();
            file.write_all(with.as_bytes()).expect("could not write");

            let mut file = File::open(&self.name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("could not read");
        }
    }
}

pub fn env_default_at(name: &str) -> PersistenceEnv {
    PersistenceEnv {
        name: name.parse().unwrap()
    }
}


pub mod disk_pers {
    use std::collections::HashMap;
    use crate::{env_default_at, MapWrapper, PersistenceEnv};
    use crate::persistence::{env_default_at, MapWrapper, PersistenceEnv};

    pub struct MapEnv {
        inner: MapWrapper,
        env: PersistenceEnv,
    }

    impl MapEnv {
        pub fn put(&mut self, key: String, value: String) -> Option<String> {
            self.update_inner();
            let value = self.inner.inner.insert(key, value);
            self.update_outer();
            value
        }

        pub fn delete(&mut self, key: String) {
            self.inner.inner.remove(&*key);
            self.update_outer();
        }

        pub fn get(&mut self, key: String) -> Option<&String> {
            self.update_inner();
            self.inner.inner.get(&*key)
        }

        fn update_inner(&mut self) {
            let from_file = self.env.load_from_local();

            let map: MapWrapper = serde_json::from_str(&*from_file).unwrap();
            for (key, value) in &map.inner {
                self.inner.inner.insert(key.clone(), value.clone());
            }
        }

        fn update_outer(&mut self) {
            let serialized = serde_json::to_string(&self.inner);
            self.env.save_to_local(&*serialized.unwrap())
        }

        pub(crate) fn get_name (&self) -> &String {&self.env.name}
    }


    pub fn create_or_load_map_env(name: &str) -> MapEnv {
        let pers_env = env_default_at(name);

        let map_wrapper = MapWrapper {
            inner: HashMap::new()
        };
        pers_env.init_env(&serde_json::to_string(&map_wrapper).unwrap());
        MapEnv {
            inner: map_wrapper,
            env: pers_env,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MapWrapper {
    inner: HashMap<String, String>,
}
