fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/resources.gresource.xml",
        "pomodoro_rsc.gresource",
    );
}
