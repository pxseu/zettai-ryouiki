use chrono::Local;

pub fn log(message: String) {
    let now = Local::now();
    let now_str = now.format("%H:%M:%S").to_string();
    println!("[{}] {}", now_str, message);
}
