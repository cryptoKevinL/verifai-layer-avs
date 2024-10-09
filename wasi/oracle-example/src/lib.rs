#[allow(warnings)]
mod bindings;

use anyhow::anyhow;
use serde_json::json;
use layer_wasi::Reactor;
use bindings::{Guest, Output, TaskQueueInput};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct TaskRequestData {
    pub object_url: String,
}

#[derive(Serialize, Debug)]
pub struct TaskResponseData {
    pub res: String,
}
struct Component;

impl Guest for Component {
    fn run_task(request: TaskQueueInput) -> Output {
        let TaskRequestData { object_url } = serde_json::from_slice(&request.request)
            .map_err(|e| anyhow!("Could not deserialize input request from JSON: {}", e))
            .unwrap();
        
        let api_url = "https://api.aiornot.com/v1/reports/image";
        let auth_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6Ijc0Y2M1OTFkLThhN2QtNDQyNi1iNzk3LTljODQ1YTAxMTM0YyIsInVzZXJfaWQiOiI3NGNjNTkxZC04YTdkLTQ0MjYtYjc5Ny05Yzg0NWEwMTEzNGMiLCJhdWQiOiJhY2Nlc3MiLCJleHAiOjAuMH0.0jNxXFIYPhYLxlTYjstUodxw4AeBLNHS4AWWIcxXoUs";
        let reactor = Reactor::default();
        
        let mut req = reactor.request(api_url).context("Failed to create request")?;
        req.set_method("POST");
        req.headers_mut().append("Authorization", format!("Bearer {}", auth_token));
        req.headers_mut().append("Content-Type", "application/json");
        req.headers_mut().append("Accept", "application/json");

        let body = json!({
            "object": object_url
        });

        req.set_body(body.to_string());

        let res = reactor.send(req).context("Failed to send request")?;

        // match res.status() {
        //     200 => res.into_body().context("Failed to parse response body"),
        //     status => Err(anyhow!("Unexpected status code: {}", status)),
        // }

        println!("AI image or not = {}", res);

        Ok(serde_json::to_vec(&TaskResponseData { res })
            .map_err(|e| anyhow!("Could not serialize output data into JSON: {}", e))
            .unwrap())
    }
}

bindings::export!(Component with_types_in bindings);
