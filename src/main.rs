extern crate rustbox;

fn main() {
    rustbox::init();
    rustbox::print(1, 1, rustbox::Bold, rustbox::White, rustbox::Black, "Hello, World!".to_string());
    rustbox::present();

    std::io::timer::sleep(std::time::Duration::milliseconds(1000));

    rustbox::shutdown();
}
