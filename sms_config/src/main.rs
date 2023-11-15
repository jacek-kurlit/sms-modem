fn main() {
    let result = sms_config::init();
    println!("{:?}", result);
    println!("{:?}", sms_config::get());
}
