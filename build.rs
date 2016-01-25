extern crate gcc;

fn main() {
    gcc::compile_library("libws2811.a", &["src/headers/ws2811.c",
                                          "src/headers/mailbox.c",
                                          "src/headers/dma.c",
                                          "src/headers/pwm.c",
                                          "src/headers/rpihw.c",]);
}
