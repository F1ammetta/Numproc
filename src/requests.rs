use std::sync::{Arc, Mutex};

async fn request(times: Arc<Mutex<Vec<f32>>>) {
    let starttime = std::time::Instant::now();
    let client = reqwest::Client::new();
    let _res = client
        .get("https://kwak.sytes.net/v0/all")
        .timeout(std::time::Duration::from_secs(100))
        .send()
        .await
        .unwrap();
    let endtime = std::time::Instant::now();
    let duration = endtime.duration_since(starttime);
    let duration = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9;
    let mut times = times.lock().unwrap();
    times.push(duration);
}

#[tokio::main]
async fn main() {
    let mut nums: Vec<f32> = Vec::new();
    let x = (10..=500).step_by(10).collect::<Vec<u32>>();
    for i in &x {
        nums.push(ping(*i).await);
    }
    let mut table = String::new();
    for i in 0..x.len() {
        table.push_str(&format!("{},{}\n", x[i], nums[i]));
    }
    std::fs::write("data.txt", table).unwrap();
}

async fn ping(n: u32) -> f32 {
    let times: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    for _ in 0..n {
        let times = times.clone();
        tokio::spawn(async move {
            request(times).await;
        });
    }
    while times.lock().unwrap().len() < n as usize {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    let times = times.lock().unwrap();
    let sum: f32 = times.iter().sum();
    let avg = sum / times.len() as f32;
    println!("Number of concurrent requests: {}", n);
    println!("Average time per request: {} seconds", avg);
    avg
}
