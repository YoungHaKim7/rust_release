fn main() {
    cc::Build::new().file("c_src/add.c").compile("add");
}
