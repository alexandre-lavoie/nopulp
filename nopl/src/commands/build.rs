use std::process::{Stdio, Command};
use std::path::Path;
use std::fs::{remove_file, remove_dir_all, read_to_string, File, read}; // copy
use std::io::Write;
use colored::Colorize;
use minifier::js::minify;

/// Builds a nopulp application.
/// 
/// Uses wasm-pack with additional packing.
pub fn build() {
    println!("{}: Packing Wasm...", "[BUILD]".magenta());

    let mut cmd = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-name", "nopl"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Wasm-pack error.");
        
    cmd.wait().expect("Wasm-pack error.");

    println!("{}: Building nopl...", "[BUILD]".magenta());

    let nopl_html = Path::new("./nopl.html");

    if nopl_html.exists() {
        remove_file(nopl_html).expect("./nopl.html should be deletable if it exists.");
    }

    let wasm = base64::encode(read("./pkg/nopl_bg.wasm").expect("./pkg/nopl_bg.wasm should exist in pkg."));

    let mut js = read_to_string(Path::new("./pkg/nopl.js")).expect("./pkg/nopl.js should exist in pkg.");

    js = js
        .replace("export default init;", "init()")
        .replace("export function", "function")
        .replace("import.meta.url.replace(/\\.js$/, '_bg.wasm');", format!("Uint8Array.from(atob(\"{}\"), c => c.charCodeAt(0));", wasm).as_str())
        .replace("input = fetch(input)", "input = input");

    // Activate to debug javascript.
    //File::create("./nopl.js")
    //    .expect("Should be able to create ./nopl.js")
    //    .write_all(js.as_bytes())
    //    .expect("Should be able to write to ./nopl.js");

    js = minify(&js);

    File::create("./nopl.html")
        .expect("Should be able to create ./nopl.html")
        .write_all(format!("<script>{}</script>", js).as_bytes())
        .expect("Should be able to write to ./nopl.html");

    // Activate to debug wasm.
    //copy("./pkg/nopl_bg.wasm", "./nopl.wasm")
    //    .expect("Should be able to copy ./pkg/nopl_bg.wasm to ./nopl.wasm");

    remove_dir_all("./pkg")
        .expect("Should be able to delete ./pkg");

    println!("{}: Done building nopl!", "[BUILD]".magenta());
}