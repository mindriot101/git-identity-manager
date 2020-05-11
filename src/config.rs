use anyhow::Result;
use git2::Config;

pub(crate) trait Entry {
    fn value(&self) -> Option<&str>;
    fn name(&self) -> Option<&str>;
}

impl<'cfg> Entry for git2::ConfigEntry<'cfg> {
    fn value(&self) -> Option<&str> {
        git2::ConfigEntry::value(self)
    }

    fn name(&self) -> Option<&str> {
        git2::ConfigEntry::name(self)
    }
}

struct EntryIterator;

impl<'a> Iterator for &'a EntryIterator {
    type Item = Result<Box<dyn Entry>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub(crate) trait GitConfig {
    fn get_string(&self, key: &str) -> Result<String>;
    fn get_entry(&self, key: &str) -> Result<Box<dyn Entry>>;
    fn set_str(&mut self, key: &str, value: &str) -> Result<()>;
    fn entries(
        &self,
        glob: Option<&str>,
    ) -> Result<Box<dyn Iterator<Item = Result<Box<dyn Entry>>>>>;
    fn remove(&mut self, key: &str) -> Result<()>;
}

impl GitConfig for Config {
    fn get_string(&self, key: &str) -> Result<String> {
        Config::get_string(self, key).map_err(From::from)
    }
    fn set_str(&mut self, key: &str, value: &str) -> Result<()> {
        Config::set_str(self, key, value).map_err(From::from)
    }

    fn entries(
        &self,
        glob: Option<&str>,
    ) -> Result<Box<dyn Iterator<Item = Result<Box<dyn Entry>>>>> {
        todo!()
    }

    fn remove(&mut self, key: &str) -> Result<()> {
        Config::remove(self, key).map_err(From::from)
    }

    fn get_entry(&self, key: &str) -> Result<Box<dyn Entry>> {
        Config::get_entry(self, key)
            .map(|e| {
                let b: Box<dyn Entry> = Box::new(e);
                b
            })
            .map_err(From::from)
    }
}
