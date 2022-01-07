#[cfg(target_os = "windows")]
pub fn setup(app_name: &str, release_server: &str) {
    let install_location: &std::path::Path = std::path::Path::new(
        format!("{}/{}", std::env::var("APPDATA").unwrap(), app_name).as_str(),
    );
    if !install_location.exists() {
        std::fs::create_dir_all(install_location)
    }
    std::env::set_current_dir(install_location);
    let mut reqwest_client_builder = reqwest::Client::builder();
    reqwest_client_builder.user_agent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    let reqwest_client = reqwest_client_builder.build()?;
    reqwest_client.get(format!("{}{}-windows.exe.sha256sum", release_server, app_name));
    return
}
