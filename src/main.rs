extern crate redisserver_sys;

fn main() {
    unsafe {
        redisserver_sys::sds_malloc(10);
    }
    println!("Hello, world!");
}
