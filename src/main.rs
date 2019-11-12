#[macro_use]
extern crate serde_json;
extern crate base64;
extern crate ulid;

use kube::{
    api::{Api, Informer, Object, PostParams, RawApi, WatchEvent},
    client::APIClient,
    config,
};

fn main() -> Result<(), failure::Error> {
    let kubeconfig = config::load_kube_config()
        .or_else(|_| config::incluster_config())
        .expect("kubeconfig failed to load");
    let client = APIClient::new(kubeconfig);
    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
    let project = std::env::var("PROJECT").expect("PROJECT env var is required");
    let sleep_time = std::time::Duration::from_secs(60 * 5);

    loop {
        std::thread::sleep(sleep_time);
        let secret = generate_secret(project.as_str());
        let data = serde_json::to_vec(&secret)?;
        let pp = PostParams::default();
        match Api::v1Secret(client).within(namespace.as_str()).create(&pp, data)
        {
            Ok(_) => println!("Sent Brigade event"),
            Err(e) => println!("Error sending event: {}", e),
        };
    }
}

fn generate_secret(project: &str) -> serde_json::Value {
    let uid = ulid::Ulid::new().to_string().to_ascii_lowercase();
    let name = format!("buck-{}", uid);
    let encoded_payload = serde_json::to_string(payload).unwrap_or_else(|_| "".to_string());

    // Currently have not imlemented clone_url or log_level
    json!({
        "apiVersion": "v1",
        "kind": "Secret",
        "metadata": {
            "name": name,
            "labels": {
                "project": project,
                "build": uid.as_str(),
                "component": "build"
            }
        },
        "type": "brigade.sh/build",
        "data": {
            "event_provider": base64::encode("buck"),
            "event_type": base64::encode("interval"),
            "project_id": base64::encode(project),
            "build_name": base64::encode(project),
            "build_id": base64::encode(uid.as_str()),
            "payload": base64::encode(encoded_payload.as_str()),
            "commit_ref": base64::encode("master")
        }
    })
}