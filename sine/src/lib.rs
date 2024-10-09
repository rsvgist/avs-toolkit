#[allow(warnings)]
mod bindings;

use anyhow::anyhow;
use bindings::{Guest, Output, TaskQueueInput};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TaskRequestData {
    pub x: u64,
}

#[derive(Serialize, Debug)]
pub struct TaskResponseData {
    pub y: u64,
}
struct Component;




impl Guest for Component {
    fn run_task(request: TaskQueueInput) -> Output {
        // 1. Accepts a TaskRequestData containing a number x
        let TaskRequestData { x } = serde_json::from_slice(&request.request)
            .map_err(|e| anyhow!("Could not deserialize input request from JSON: {}", e))
            .unwrap();
        // 2. Logic to square the number
        let y = x * x; 
        println!("{}^2 = {}", x, y);

        // 3. Returns result with y as TaskResponseData 
        Ok(serde_json::to_vec(&TaskResponseData { y })
            .map_err(|e| anyhow!("Could not serialize output data into JSON: {}", e))
            .unwrap())
    }
}

bindings::export!(Component with_types_in bindings);
