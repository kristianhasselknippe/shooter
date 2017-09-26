extern crate notify;

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::path::{PathBuf,Path};
use std::time::Duration;
use std::thread::spawn;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver,Sender,channel};

use std::env::current_dir;

fn script_watcher(scripts_path: &Path, event_sender: Sender<DebouncedEvent>) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(0)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(scripts_path, RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    spawn(move || {
        println!("Starting file watcher loop");
        loop {
            for ev in rx.try_iter() {
                println!("Got file event - {:?}", ev);
                event_sender.send(ev);
            }

        }
    });
    Ok(())
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
        let (tx, rx) = channel();
        let mut cd = current_dir().unwrap();
        cd.push(scripts_path);
        println!("Path dir: {:?}", &cd);
        script_watcher(&cd, tx).unwrap();
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
        for event in self.rx.try_iter() {
            println!("Got an event");
            println!("{:?}", event);
        }
    }
}
