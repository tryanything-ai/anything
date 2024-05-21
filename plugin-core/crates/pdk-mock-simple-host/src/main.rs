// create a main
// have it load plugin from file
// have it call register on them
// have it call execute on them

use extism::*;

fn main() {
    let url =
        Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");
    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();
    let res = plugin
        .call::<&str, &str>("count_vowels", "Hello, world!")
        .unwrap();
    println!("{}", res);
}
