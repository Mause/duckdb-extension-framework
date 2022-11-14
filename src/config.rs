use std::{
    ffi::CString,
    ptr::{addr_of_mut, null_mut},
};

use crate::{
    check,
    duckly::{
        duckdb_config, duckdb_config_count, duckdb_create_config, duckdb_destroy_config,
        duckdb_get_config_flag, duckdb_set_config,
    },
};

pub struct Config(pub(crate) duckdb_config);

pub fn get_configs() -> ConfigList {
    ConfigList { idx: 0 }
}

pub struct ConfigItem {
    pub name: String,
    pub desc: String,
}

pub struct ConfigList {
    idx: usize,
}
impl Iterator for ConfigList {
    type Item = ConfigItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().ok()
    }
}
impl ConfigList {
    fn try_next(&mut self) -> Result<ConfigItem, Box<dyn std::error::Error>> {
        let name = CString::new("")?;
        let desc = CString::new("")?;

        let mut name_ptr = name.as_ptr();
        let mut desc_ptr = desc.as_ptr();
        check!(unsafe {
            duckdb_get_config_flag(self.idx, addr_of_mut!(name_ptr), addr_of_mut!(desc_ptr))
        });
        self.idx += 1;

        Ok(ConfigItem {
            name: name.to_str()?.to_owned(),
            desc: desc.to_str()?.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        unsafe { duckdb_config_count() }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut out_config: duckdb_config = null_mut();
        check!(unsafe { duckdb_create_config(addr_of_mut!(out_config)) });
        Ok(Self(out_config))
    }
    pub fn set_flag(&mut self, name: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let name = CString::new(name)?;
        let value = CString::new(value)?;
        check!(unsafe { duckdb_set_config(self.0, name.as_ptr(), value.as_ptr()) });
        Ok(())
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { duckdb_destroy_config(addr_of_mut!(self.0)) };
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{get_configs, Config, ConfigItem};

    #[test]
    fn test_config_list() {
        let lst = get_configs();

        let lst: Vec<ConfigItem> = lst.collect();

        assert!(lst.len() > 0);
    }

    #[test]
    fn test_config_populate() -> Result<(), Box<dyn Error>> {
        let mut config = Config::new()?;

        config.set_flag("access_mode", "READ_ONLY")?;

        Ok(())
    }
}
