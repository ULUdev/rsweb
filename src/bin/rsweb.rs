use rsweb::tp;
use rsweb::http;

fn main() {
    let threadpool = tp::ThreadPool::new(4);
    for _ in 0..15 {
        threadpool.execute(|| {
            println!("ok");
            std::thread::sleep_ms(1000);
        });
    }
}
