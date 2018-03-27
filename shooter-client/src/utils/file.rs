use std::env::current_dir;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: &Path) -> Result<String,()> {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    Ok(s)
}
    
const ASSETS_FOLDER: &str = "assets";

///Loads an asset from the asset folder
pub fn read_asset(asset_name: &str) -> Result<String,()> {
    if let Ok(mut p) = current_dir() {
        p.push(ASSETS_FOLDER);
        p.push(asset_name);
        println!("Asset path: {:?}", p);
        read_file(&p)
    } else {
        Err(())
    }
}
