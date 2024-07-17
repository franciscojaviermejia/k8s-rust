use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, PostParams},
    Client,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Create an API client for Pods in the namespace
    let pods: Api<Pod> = Api::namespaced(client, "storefront-renderer-staging-east-2");

    let pod_json = serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "name": "dummy-pod"
        },
        "spec": {
            "containers": [{
                "name": "dummy-container",
                "image": "busybox",
                "command": ["sleep", "3600"]
            }]
        }
    });

    let pod: Pod = serde_json::from_value(pod_json)?;

    println!("Creating dummy pod in the namespace...");
    let pod = pods.create(&PostParams::default(), &pod).await?;
    println!("Pod created: {}", pod.metadata.name.unwrap());

    println!("Waiting for 1 minute...");
    sleep(Duration::from_secs(60)).await;

    println!("Deleting dummy pod...");
    pods.delete("dummy-pod", &DeleteParams::default()).await?;
    println!("Pod deleted");

    Ok(())
}
