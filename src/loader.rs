use std::{collections::HashMap, error::Error, mem::MaybeUninit};

use derive_more::Display;
use dlopen2::wrapper::Container;
use dlopen2::wrapper::WrapperApi;
use libloading::{Library, Symbol};
use smallbox::{space, SmallBox};
use thiserror::Error;

use crate::CBox;
use crate::{capi::plugin, command::{Command, Response, ResponseError}, event::Events, plugin::{InitData, Plugin}};

#[derive(Default)]
pub struct PluginLoader2 {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libs: Vec<Container<PluginApi>>,
    init_data: InitData,
}

#[derive(WrapperApi)]
struct PluginApi {
    load_plugin: extern "C" fn(id: InitData) -> CBox<dyn Plugin>,
}

impl PluginLoader2 {

    pub fn load_plugin(&mut self, path:&str) -> Result<(), LoaderError> {
        let lib: Container<PluginApi> = unsafe {
            Container::load(path)?
        };

        let wrapper = lib.load_plugin(self.init_data.clone());
        let plugin = wrapper.to_box();

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


pub struct PluginLoader {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libs: Vec<Library>,
    init_data: InitData,
}

#[derive(Debug, Error, Display)]
pub enum LoaderError {
    LibLoad(#[from] libloading::Error),
    #[display("Duplicate Plugin {name} versions: [{ver1}, {ver2}]")]
    DuplicatePlugin {
        name: String,
        ver1: String,
        ver2: String
    },
    DlOpen(#[from] dlopen2::Error)
}

// impl PluginLoader {
//     pub fn new() -> Self {
//         PluginLoader { plugins: HashMap::new(), libs:Vec::new(), init_data: InitData { events: CEvents::from_events(Events::new()) } }
//     }

//     // pub fn load_c_plugin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
//     //     unsafe {
//     //         let lib = Library::new(path)?;
//     //         let plugin = CPlugin::new(lib)?;
//     //         let name = plugin.name().to_string();
//     //         if self.plugins.contains_key(&name) {
//     //             todo!()
//     //         }
//     //         self.plugins.insert(name, Box::new(plugin));
//     //     }
//     //     Ok(())
//     // }

    

//     pub fn load_plugin(&mut self, path: &str) -> Result<(), LoaderError> {
//         unsafe {
//             let lib = Library::new(path)?;

//             let ctor: Symbol<LoadFunc> = lib.get(LOAD_FUNC_NAME)?;
//             let plugin = ctor(self.init_data.clone());
//             let name = plugin.name().to_string();
//             if self.plugins.contains_key(&name) {
//                 let ver1 = self.plugins.get(&name).expect("unreachable!").version().to_string();
//                 let ver2 = plugin.version().to_string();

//                 return Err(LoaderError::DuplicatePlugin { name, ver1, ver2 });
//             }
//             self.plugins.insert(name, plugin);
//             self.libs.push(lib);
//         }
//         Ok(())
//     }
// }