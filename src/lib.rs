mod bsd;
mod linux;
mod mac;
mod windows;

/// Hedgehog's main function
/// Call this at the very start of your program, will call the entire update sequence (including moving the binary) according to your OS
/// app_name should be... the name of your app, release_server is a URL to your release server
/// The release server folder should have the latest version of your app for this OS, configured as so: `<appname>-<osname>-<arch>` with the `exe` extension for windows.
/// `appname` can be whatever you want, but must remain consistent throughout updates. `osname` should be `bsd`, `linux`, `macOS`, or `windows`, and `arch` should be the Cargo arch
/// There MUST also be a file with the extension .sha256sum appended onto the filename of the app, so it can detect if it's been updated and validate the download.
pub fn update(app_name: &str, release_server: &str) {
    #[cfg(target_os = "windows")]
    windows::setup(app_name, release_server);
    #[cfg(target_os = "macos")]
    mac::setup(app_name, release_server);
    #[cfg(target_os = "linux")]
    linux::setup(app_name, release_server);
    #[cfg(any(
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    bsd::setup(app_name, release_server);
}
