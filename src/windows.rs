use sha2::Digest;

#[cfg(target_os = "windows")]
pub fn setup(app_name: &str, release_server: &str) {
    let path_string = format!("{}/{}", std::env::var("APPDATA").unwrap(), app_name);
    let install_location: &std::path::Path = std::path::Path::new(&path_string);
    let executable_path = std::path::PathBuf::from(format!(
        "{}{}{}-windows-{}.exe",
        path_string,
        release_server,
        app_name,
        std::env::consts::ARCH
    ));
    println!("{:?}", install_location);
    if !install_location.exists() {
        if let Err(err) = std::fs::create_dir_all(install_location) {
            panic!("{:?}", err);
        }
    }
    if let Err(err) = std::env::set_current_dir(install_location) {
        panic!("{:?}", err);
    }
    let reqwest_client = reqwest::blocking::Client::builder()
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap(); // If the client fails to build something else is very wrong.
    let request = reqwest_client.get(format!(
        "{}{}-windows-x86_64.exe.sha256sum",
        release_server, app_name
    ));
    let response = reqwest_client
        .execute(request.build().unwrap())
        .unwrap()
        .text()
        .unwrap()
        .trim()
        .to_string();
    println!("{:#?}", response);
    if executable_path.exists() {
        let mut hasher = sha2::Sha256::new();
        let mut file = std::fs::File::open(executable_path).unwrap();
        std::io::copy(&mut file, &mut hasher).unwrap();
        let hash = hasher
            .finalize()
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        println!("Hasher result: `{:#?}`, response: `{}`", hash, response);
        if hash == response {
            println!("We already have the latest version, exiting...");
            return;
        }
    } else {
        println!("{:?}", std::env::current_exe().unwrap());
        std::fs::rename(std::env::current_exe().unwrap(), executable_path).unwrap();
        setup(app_name, release_server);
    }
}
