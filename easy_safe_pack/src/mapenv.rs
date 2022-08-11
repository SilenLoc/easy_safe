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

/// creates a "environment" on the disk, it just wraps writing and reading from a file
pub fn env_default_at(name: &str) -> PersistenceEnv {
    PersistenceEnv {
        name: name.parse().unwrap()
    }
}


pub mod disk_pers {
    use std::collections::HashMap;

    use crate::mapenv::{env_default_at, MapWrapper, PersistenceEnv};

    pub struct MapEnv {
        inner: MapWrapper,
        env: PersistenceEnv,
    }

    impl MapEnv {
        /// put will first try to load the newest file,
        /// puts the value into the in memory map and then saves the map to disk
        /// also return the value
        pub fn put(&mut self, key: &str, value: &str) -> Option<String> {
            self.update_inner();
            let value = self.inner.inner.insert(
                key.parse().unwrap(),
                value.parse().unwrap()
            );
            self.update_outer();
            value
        }

        /// deletes the value in the in memory map and then saves the map to disk
        /// and also returns the value

        pub fn delete(&mut self, key: &str) {
            self.inner.inner.remove(key);
            self.update_outer();
        }

        /// get trys to load the map from disk and then attempts to get the value from the in memory map
        pub fn get(&mut self, key: &str) -> Option<&String> {
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

        /// returns the name of the inner environment
        pub(crate) fn get_name(&self) -> &String { &self.env.name }
    }


    /// creates a new map environment with the specified name
    /// the name corresponds to the file saved and loaded in your filesystem
    /// If the file is already there it will load from that file
    ///
    /// This means you can always come back and access your file if you call it with the right name
    ///
    /// # Examples
    ///```
    ///  use easy_safe::{create_or_load_map_env, MapEnv};
    ///  let mut  map_env: MapEnv = create_or_load_map_env("somename");
    ///  map_env.put("somekey", "somevalue");
    ///  let value = map_env.get("somekey").unwrap();
    ///  assert_eq!(value, "somevalue");
    ///
    ///  let mut  same_file_map_env: MapEnv = create_or_load_map_env("somename");
    ///  let also_the_value = same_file_map_env.get("somekey").unwrap();
    ///  assert_eq!(value, "somevalue");
    ///```
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

/// the wrapping struct to the map, this is what will be saved as json on the disk
/// this is mostly to not break interfaces and it should make it always possible
/// to load old files if needed
#[derive(Serialize, Deserialize)]
struct MapWrapper {
    inner: HashMap<String, String>,
}