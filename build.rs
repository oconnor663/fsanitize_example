fn main() {
    let mut build = cc::Build::new();
    build.file("src/oops.c");
    build.flag("-fsanitize=address,undefined");
    build.compile("oops");
}
