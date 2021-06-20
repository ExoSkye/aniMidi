extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("portmidi").define("BUILD_TESTING","NO").build_target("install").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=portmidi");
}
