use std::{collections::HashMap, error::Error, mem::MaybeUninit};

use libloading::{Library, Symbol};

use crate::{capi::plugin::CPlugin, command::{Command, Response, ResponseError}, plugin::Plugin};



pub struct PluginLoader {
    plugins: HashMap<String, Box<dyn Plugin>>,
    libs: Vec<Library>
}

impl PluginLoader {
    pub fn new() -> Self {
        PluginLoader { plugins: HashMap::new(), libs:Vec::new() }
    }

    pub fn load_c_plugin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let plugin = CPlugin::new(lib)?;
            let name = plugin.name().to_string();
            if self.plugins.contains_key(&name) {
                todo!()
            }
            self.plugins.insert(name, Box::new(plugin));
        }
        Ok(())
    }

    pub fn load_plugin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        unsafe {
            let lib = Library::new(path)?;
            let ctor: Symbol<fn(&mut MaybeUninit<Box<dyn Plugin>>)> = lib.get(b"create_plugin")?;
            let mut plugin = MaybeUninit::uninit();
            ctor(&mut plugin);
            let plugin = plugin.assume_init();
            let name = plugin.name().to_string();
            if self.plugins.contains_key(&name) {
                todo!()
            }
            self.plugins.insert(name, plugin);
            self.libs.push(lib);
        }
        Ok(())
    }

    pub fn execute_command<F>(&self, command: &Command, cond: F) -> HashMap<&str, Result<Response, ResponseError>> 
    where F: Fn(&dyn Plugin) -> bool {
        let mut result = HashMap::new();
        for plugin in &self.plugins {
            if cond(&**plugin.1) {
                result.insert(plugin.0.as_str(), plugin.1.execute(command));
            }
        }
        result
    }
}