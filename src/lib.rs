use sha2::Digest;
// Why so few imports?
// To me, the code is more clear if i know exactly what module everything is in.

/// Hedgehog's main function
/// Call this at the very start of your program, will call the entire update sequence (including moving the binary) according to your OS
/// app_name should be... the name of your app, release_server is a URL to your release server folder with a trailing slash.
/// The release server folder should have the latest version of your app for this OS, configured as so: `<appname>-<osname>-<arch>` with the `exe` extension for windows.
/// `appname` can be whatever you want, but must remain consistent throughout updates. `osname` should be from https://doc.rust-lang.org/std/env/consts/constant.OS.html and `arch` should be from https://doc.rust-lang.org/std/env/consts/constant.ARCH.html
/// There MUST also be a file with the extension .sha256sum appended onto the filename of the app, so it can detect if it's been updated and validate the download.
pub fn update(app_name: &str, release_server: &str) {
    #[cfg(target_os = "windows")]
        let path_string = format!("{}/{}/", std::env::var("APPDATA").unwrap(), app_name);
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        let path_string = format!("{}/.{}/", std::env::var("HOME").unwrap(), app_name);
    let install_location: &std::path::Path = std::path::Path::new(&path_string);
    #[cfg(target_family = "windows")]
        let executable_path = std::path::PathBuf::from(format!(
        "{}{}-windows-{}.exe",
        path_string,
        app_name,
        std::env::consts::ARCH
    ));
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        let mut executable_path = std::path::PathBuf::from(format!(
        "{}{}-{}-{}",
        path_string,
        app_name,
        std::env::consts::OS,
        std::env::consts::ARCH
    ));
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
            "{}/{}, {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            std::env::consts::OS
        ))
        .build()
        .unwrap();
    #[cfg(target_family = "windows")]
        let request = reqwest_client.get(format!(
        "{}{}-windows-{}.exe.sha256sum",
        release_server,
        app_name,
        std::env::consts::ARCH
    ));
    #[cfg(target_family = "unix")]
        let request = reqwest_client.get(format!(
        "{}{}-{}-{}.sha256sum",
        release_server,
        app_name,
        std::env::consts::OS,
        std::env::consts::ARCH
    ));
    let response = reqwest_client
        .execute(request.build().unwrap())
        .unwrap()
        .text()
        .unwrap()
        .trim()
        .to_string()
        .to_lowercase();
    if executable_path.exists() {
        let mut hasher = sha2::Sha256::new();
        let mut file = std::fs::File::open(executable_path.clone()).unwrap();
        std::io::copy(&mut file, &mut hasher).unwrap();
        let hash = hasher
            .finalize()
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        println!("Hasher result: `{:#?}`, response: `{}`", hash, response);
        if hash == response {
            println!("We already have the latest version, exiting Hedgehog...");
        } else {
            #[cfg(target_family = "windows")]
                let download_request = reqwest_client.get(format!(
                "{}{}-windows-{}.exe",
                release_server,
                app_name,
                std::env::consts::ARCH
            ));
            #[cfg(target_family = "unix")]
                let download_request = reqwest_client.get(format!(
                "{}{}-{}-{}",
                release_server,
                app_name,
                std::env::consts::OS,
                std::env::consts::ARCH
            ));
            let mut new_executable_path = executable_path;
            new_executable_path.push(".tmp");
            let mut update_file_path = std::fs::File::create(new_executable_path).unwrap();
            let file_object = reqwest_client.execute(download_request.build().unwrap()).unwrap().bytes().unwrap();
            std::io::copy(&mut file_object.as_ref(), &mut update_file_path).unwrap();
        }
    } else {
        println!(
            "Moving from {:?} to {:?}",
            std::env::current_exe().unwrap(),
            executable_path
        );
        std::fs::rename(std::env::current_exe().unwrap(), executable_path.clone()).unwrap();
        std::process::Command::new(executable_path);
        std::process::exit(0)
    }
}
