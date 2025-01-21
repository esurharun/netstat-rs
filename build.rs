fn main() {
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("src/integrations/osx/ext/get_netstat.c") // Path to your C file
            .compile("get_netstat"); // Name of the compiled library

        println!("cargo:rerun-if-changed=src/integrations/osx/ext/get_netstat.c");
    }
}
