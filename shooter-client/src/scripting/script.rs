extern crate notify;

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::path::{PathBuf,Path};
use std::time::Duration;
use std::thread::spawn;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver,Sender,channel};

use std::env::current_dir;

fn script_watcher(scripts_path: &Path, sender: Sender<DebouncedEvent>) {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    println!("Watching path: {:?}", scripts_path);

    watcher.watch(scripts_path, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                sender.send(event);
            },
            Err(e) => {
                println!("watch error: {:?}", e);
                break;
            },
        }
    }
}

#[derive(Hash,Eq,PartialEq)]
pub struct ScriptId(i32);
pub struct Script {
    id: ScriptId,
    file: PathBuf,
}

pub struct ScriptWatcher {
    script_id_counter: i32,
    script_paths: HashMap<ScriptId, PathBuf>,
    rx: Receiver<DebouncedEvent>,
}

impl ScriptWatcher {
    pub fn new(scripts_path: &Path) -> ScriptWatcher {
        let mut cd = current_dir().unwrap();
        cd.push(scripts_path);
        println!("Path dir: {:?}", &cd);
        let (tx,rx) = channel();
        spawn(move ||{
            script_watcher(&cd, tx);
        });

        ScriptWatcher {
            script_id_counter: 0,
            script_paths: HashMap::new(),
            rx: rx,
        }
    }

    pub fn new_script_from_file(&mut self, path: &Path) -> Script {
        let id = ScriptId(self.script_id_counter);
        self.script_id_counter += 1;
        Script {
            id: id,
            file: path.to_path_buf(),
        }
    }

    pub fn tick(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => { break; },
            }
        }
    }
}
