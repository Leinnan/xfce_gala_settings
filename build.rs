use freedesktop_desktop_entry::{Application, DesktopEntry, DesktopType};
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

const APPID: &str = "xfce_gala_settings";

fn main() {
    let exec_path = Path::new("/usr").join("bin").join(APPID);
    let exec = exec_path.as_os_str().to_str().expect("prefix is not UTF-8");

    let mut desktop = File::create(["target/", APPID, ".desktop"].concat().as_str())
        .expect("failed to create desktop entry file");

    let entry = DesktopEntry::new(
        "XFCE Gala Settings",
        APPID,
        DesktopType::Application(
            Application::new(&["System", "GTK"], exec)
                .keywords(&["xfce", "gala" ,"settings", "config"])
                .startup_notify(),
        ),
    )
    .comment("Gala WM settings for XFCE")
    .generic_name("Gala XFCE Settings");

    desktop.write_all(entry.to_string().as_bytes());
}