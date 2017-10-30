extern crate notify;

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use self::notify::DebouncedEvent::*;
use super::lua::*;
use std::path::{PathBuf,Path};
use std::time::Duration;
use std::thread::spawn;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver,Sender,channel};
use std::fs::File;

use std::rc::Rc;

fn script_watcher(scripts_path: &Path, sender: Sender<DebouncedEvent>) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(100)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    println!("Watching path: {:?}", scripts_path);

    watcher.watch(scripts_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                sender.send(event).unwrap();
            },
            Err(e) => {
                println!("watch error: {:?}", e);
                break;
            },
        }
    }
}

#[derive(Hash,Eq,PartialEq,Clone,Copy,Debug)]
pub struct ScriptId(i32);
#[derive(Clone,Debug)]
pub struct Script {
    id: ScriptId,
    path: PathBuf,
    lua: Rc<Lua>,
}

impl Script {
    fn new(id: ScriptId, path: &Path, lua: &Rc<Lua>) -> Script {
        Script {
            id: id,
            path: path.to_path_buf(),
            lua: lua.clone(),
        }
    }

    pub fn get_string_id(&self) -> String {
        format!("bs{}", self.id.0)
    }

    pub fn set_field(&self, name: &str, val: &LuaType) {
        self.lua.set_global(&format!("__entity_scripts.{}.{}", self.get_string_id(), name), val);
    }

    fn load_as_module(&self) {
        println!("Loading module: {:?}", self.path.file_name().unwrap());
        self.lua.load_as_module(&self.path);
    }

    fn load_as_script(&self, id: &str) {
        println!("Loading script: {:?}", self.path.file_name().unwrap());
        self.lua.load_as_script(&self.path, id);
    }
}

#[derive(Debug)]
pub struct ScriptWatcher {
    script_id_counter: i32,
    script_paths: HashMap<ScriptId, Script>,
    rx: Receiver<DebouncedEvent>,
}

impl ScriptWatcher {
    pub fn new(scripts_path: &Path) -> ScriptWatcher {
        let (tx,rx) = channel();
        let dir = scripts_path.to_path_buf();
        spawn(move ||{
            script_watcher(&dir, tx);
        });

        ScriptWatcher {
            script_id_counter: 0,
            script_paths: HashMap::new(),
            rx: rx,
        }
    }

    pub fn new_module(&mut self, path: &Path, lua: &Rc<Lua>) {
        let id = ScriptId(self.script_id_counter);
        self.script_id_counter += 1;

        let ret = Script::new(id,path, lua);
        self.script_paths.insert(id, ret.clone());
        ret.load_as_module();
    }

    pub fn new_script(&mut self, path: &Path, lua: &Rc<Lua>) -> Script {
        let id = ScriptId(self.script_id_counter);
        self.script_id_counter += 1;

        let ret = Script::new(id,path, lua);
        ret.load_as_script(&ret.get_string_id());
        self.script_paths.insert(id, ret.clone());
        ret
    }

    pub fn tick(&self) {
        loop {
            match self.rx.try_recv() {
                Ok(event) => {
                    //println!("{:?}", event);
                    match event {
                        NoticeWrite(_) => { },
                        NoticeRemove(_) => {

                        },
                        Create(_) => { },
                        Write(path) => {
                            for (_,p) in &self.script_paths {
                                if path.ends_with(&p.path) {
                                    p.load_as_module();
                                }
                            }
                        },
                        Chmod(_) => { },
                        Remove(_) => { },
                        Rename(_, _) => { },
                        Rescan => { },
                        Error(_, _) => { },
                    }

                },
                Err(_) => { break; },
            }
        }
    }
}
