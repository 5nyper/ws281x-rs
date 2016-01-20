extern crate gcc;

fn main() {
    gcc::Config::new().file("src/headers/ws2811.c").compile("libws2811.a");
}
