use std::println;

use tokio::time::{sleep, Duration};

use crate::client::process_get_job;

pub async fn listen_for_changes(job_id: &str) {
    println!("Listening for changes on job {}", job_id);
    loop {
        let response = process_get_job(job_id.to_string()).await.unwrap();
        println!("{:#?}", response.job_status.job_state);
        sleep(Duration::from_secs(2)).await;
    }
}