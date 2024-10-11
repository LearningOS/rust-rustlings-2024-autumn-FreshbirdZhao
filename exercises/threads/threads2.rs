// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 包装 Mutex 包装 JobStatus，以实现线程间的共享和互斥访问
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        // 克隆 Arc，以便每个线程都可以访问共享状态
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // 休眠一段时间模拟工作
            thread::sleep(Duration::from_millis(250));
            // 通过锁来更新共享状态
            let mut num = status_shared.lock().unwrap();
            // TODO: You must take an action before you update a shared value
            num.jobs_completed += 1;
            // 锁会在 num 离开作用域时自动释放
        });
        handles.push(handle);
    }
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    // 访问共享状态并打印结果
    let result = status.lock().unwrap();
    // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
    // anything interesting in the output? Do you have to 'join' on all the
    // handles?
    println!("jobs completed {}", result.jobs_completed);
}