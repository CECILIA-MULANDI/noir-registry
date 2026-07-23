/// Gets the registry URL from args, env var, or default
pub fn get_registry_url(args_registry: Option<String>) -> String {
    args_registry
        .or_else(|| std::env::var("NOIR_REGISTRY_URL").ok())
        .unwrap_or_else(|| "https://noir-registry.fly.dev/api".to_string())
}
