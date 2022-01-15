use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

type Data = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone, Copy)]
pub enum DatabaseError {
    LoadError,
    SaveError,

    ParseError,
    SerializeError,
    NoItem,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub file: PathBuf,
    pub data: Data,
}

impl Database {
    pub fn load<T>(file: T) -> Result<Database, DatabaseError>
    where
        T: AsRef<Path>,
    {
        let data = match fs::read(&file) {
            Ok(i) => i,
            Err(_) => return Err(DatabaseError::LoadError),
        };

        let parsed: Data = match bincode::deserialize(&data) {
            Ok(i) => i,
            Err(_) => return Err(DatabaseError::ParseError),
        };

        Ok(Database {
            file: file.as_ref().to_path_buf(),
            data: parsed,
        })
    }

    pub fn save(&self) -> Result<(), DatabaseError> {
        let data = match bincode::serialize(&self.data) {
            Ok(i) => i,
            Err(_) => return Err(DatabaseError::SerializeError),
        };

        match fs::write(&self.file, data) {
            Ok(i) => i,
            Err(_) => return Err(DatabaseError::SaveError),
        };

        Ok(())
    }

    pub fn new<T>(file: T) -> Self
    where
        T: AsRef<Path>,
    {
        Database {
            file: file.as_ref().to_path_buf(),
            data: HashMap::new(),
        }
    }

    pub fn table_exists<T>(&mut self, table: T) -> Option<bool>
    where
        T: std::fmt::Display,
    {
        Some(self.data.contains_key(&table.to_string()))
    }

    pub fn value_exists<T, K>(&self, table: T, key: K) -> Option<bool>
    where
        T: std::fmt::Display,
        K: std::fmt::Display,
    {
        Some(
            self.data
                .get(&table.to_string())?
                .contains_key(&key.to_string()),
        )
    }

    pub fn table<T>(&mut self, table: T) -> Option<bool>
    where
        T: std::fmt::Display,
    {
        Some(matches!(
            self.data.insert(table.to_string(), HashMap::new()),
            Some(_)
        ))
    }

    pub fn set<T, K, V>(&mut self, table: T, key: K, value: V) -> Option<()>
    where
        T: std::fmt::Display,
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        self.data
            .get_mut(&table.to_string())?
            .insert(key.to_string(), value.to_string());

        Some(())
    }

    pub fn get<T, K, O>(&self, table: T, key: K) -> Result<O, DatabaseError>
    where
        T: std::fmt::Display,
        K: std::fmt::Display,
        O: core::str::FromStr,
    {
        let item = match self.get_str(table, key) {
            Some(i) => i,
            None => return Err(DatabaseError::NoItem),
        };

        match item.parse() {
            Ok(i) => Ok(i),
            Err(_) => Err(DatabaseError::ParseError),
        }
    }

    pub fn get_set<T, K, V>(
        &mut self,
        table: T,
        key: K,
        value: Box<dyn Fn(String) -> Option<V>>,
    ) -> Option<String>
    where
        T: std::fmt::Display,
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        let table = self.data.get_mut(&table.to_string())?;
        let out = (value)(table.get(&key.to_string())?.to_owned())?.to_string();

        table.insert(key.to_string(), out.clone());

        Some(out)
    }

    pub fn get_str<T, K>(&self, table: T, key: K) -> Option<&str>
    where
        T: std::fmt::Display,
        K: std::fmt::Display,
    {
        Some(self.data.get(&table.to_string())?.get(&key.to_string())?)
    }
}
