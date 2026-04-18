// needed only for wasm32 target
// this build.rs reuse the 'metal'
// downloader to dl files in `resources` folder
// so they can be used in wasm bundle

//TODO: gate this when when we are running 
// it for wasm (yes running not building)
// build.rs will be built on source system
// check target vs build_target or something

use core::panic;
use std::path::Path;
mod download {
    include!("src/download.rs");
}

use download::*;

fn main() {
    let out_dir = "resources";
    let out_path = Path::new(out_dir);

    match Downloader::new(out_path.into()).download_if_needed() {
        Ok(_) => (),
        Err(e) => panic!("couldn't dl resources: {e}"),
    };
}
