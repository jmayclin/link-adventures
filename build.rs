fn main() {
    println!("cargo:rerun-if-changed=lib/container.c");

    cc::Build::new()
        .file("lib/container.c")
        //.define("DEBUG_ASM", None)
    .compile("mycontainer");
}