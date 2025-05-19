use std::collections::HashMap;

use derive_more::Display;
use dlopen2::wrapper::Container;
use dlopen2::wrapper::WrapperApi;
use thiserror::Error;

use crate::FatCBox;
use crate::{plugin::{InitData, Plugin}};

#[derive(Default)]
pub struct PluginLoader {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libs: Vec<Container<PluginApi>>,
    init_data: InitData,
}

#[derive(WrapperApi)]
struct PluginApi {
    load_plugin: extern "C" fn(id: InitData) -> FatCBox,
}

impl PluginLoader {

    pub fn load_plugin(&mut self, path:&str) -> Result<(), LoaderError> {
        let lib: Container<PluginApi> = unsafe {
            Container::load(path)?
        };

        let wrapper = lib.load_plugin(self.init_data.clone());
        let plugin = unsafe {wrapper.to_box::<dyn Plugin>()};

        let name = plugin.name().to_string();
            if self.plugins.contains_key(&name) {
                let ver1 = self.plugins.get(&name).expect("unreachable!").version().to_string();
                let ver2 = plugin.version().to_string();

                return Err(LoaderError::DuplicatePlugin { name, ver1, ver2 });
            }
            self.plugins.insert(name, plugin);
            self.libs.push(lib);
        Ok(())
    }
}


#[derive(Debug, Error, Display)]
pub enum LoaderError {
    #[display("Duplicate Plugin {name} versions: [{ver1}, {ver2}]")]
    DuplicatePlugin {
        name: String,
        ver1: String,
        ver2: String
    },
    DlOpen(#[from] dlopen2::Error)
}
