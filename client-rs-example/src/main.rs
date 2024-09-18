use std::{collections::HashMap, str::FromStr};
use temporal_client::{ClientOptionsBuilder, WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core::{protos::temporal::api::common::v1::Payload, Url};

const TEMPORAL_SERVER: &str = "http://localhost:7233";
const DEFAULT_NAMESPACE: &str = "default";

#[tokio::main]
async fn main() {
    let client_options = ClientOptionsBuilder::default()
        .target_url(Url::from_str(TEMPORAL_SERVER).unwrap())
        .client_name("cute-kitty".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .expect("Unable to create client");

    // let client = Client::new(client_options, namespace);
    let client = client_options
        .connect(DEFAULT_NAMESPACE.to_string(), None)
        .await
        .unwrap();

    let mut metadata = HashMap::new();
    metadata.insert(
        "encoding".to_string(),
        b"string".to_vec()
    );

    let input = vec![Payload {
        metadata,
        data: b"Deno".to_vec(),
    }];

    let task_queue = "hello-world".to_string();
    let workflow_id = "workflow-id-123".to_string();
    let workflow_type = "example".to_string();
    let workflow_options = WorkflowOptions::default();
    let response = client
        .start_workflow(
            input,
            task_queue,
            workflow_id,
            workflow_type,
            None,
            workflow_options,
        )
        .await
        .unwrap();
    dbg!(response);
}
