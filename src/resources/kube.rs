use opentelemetry::KeyValue;

pub fn kube_env_resource() -> Vec<KeyValue> {
    let mut kvs = Vec::with_capacity(4);

    if let Ok(v) = std::env::var("KUBE_APP_PART_OF") {
        kvs.push(KeyValue::new("kube_app_part_of", v));
    }
    if let Ok(v) = std::env::var("KUBE_APP_VERSION") {
        kvs.push(KeyValue::new("kube_app_version", v));
    }
    if let Ok(v) = std::env::var("KUBE_APP_INSTANCE") {
        kvs.push(KeyValue::new("kube_app_instance", v));
    }
    if let Ok(v) = std::env::var("KUBE_APP_MANAGED_BY") {
        kvs.push(KeyValue::new("kube_app_managed_by", v));
    }

    kvs
}
