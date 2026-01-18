use std::io;

#[cfg(target_os = "linux")]
mod platform {
    use std::{
        env,
        fs,
        io,
        process::Command,
        path::PathBuf,
    };

    use dirs_next::data_dir;

    pub fn register() -> io::Result<()> {
        let data_dir = data_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No data directory found"))?;

        // ~/.local/share/applications
        let applications_dir: PathBuf = data_dir.join("applications");
        fs::create_dir_all(&applications_dir)?;

        let desktop_path = applications_dir.join("agm-url-handler.desktop");

        let exe_path = env::current_exe()?;
        let exe_path = exe_path.display().to_string();

        let desktop_file = format!(
            "[Desktop Entry]\n\
             Name=AGM URL Handler\n\
             Exec={} %u\n\
             Type=Application\n\
             Terminal=false\n\
             MimeType=x-scheme-handler/nxm;x-scheme-handler/nexusmods;\n",
            exe_path
        );

        fs::write(&desktop_path, desktop_file)?;

        Command::new("xdg-mime")
            .args([
                "default",
                "agm-url-handler.desktop",
                "x-scheme-handler/nxm",
            ])
            .status()?;

        Command::new("xdg-mime")
            .args([
                "default",
                "agm-url-handler.desktop",
                "x-scheme-handler/nexusmods",
            ])
            .status()?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use std::{env, io};
    use winreg::enums::*;
    use winreg::RegKey;

    pub fn register() -> io::Result<()> {
        let exe = env::current_exe()?.display().to_string();
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        register_scheme(&hkcu, "nxm", &exe)?;
        register_scheme(&hkcu, "nexusmods", &exe)?;

        Ok(())
    }

    fn register_scheme(
        root: &RegKey,
        scheme: &str,
        exe: &str,
    ) -> io::Result<()> {
        let path = format!("Software\\Classes\\{}", scheme);
        let (key, _) = root.create_subkey(&path)?;

        key.set_value("", &format!("URL:{} Protocol", scheme))?;
        key.set_value("URL Protocol", &"")?;

        let (cmd, _) = root.create_subkey(format!(
            "{}\\shell\\open\\command",
            path
        ))?;

        cmd.set_value("", &format!("\"{}\" \"%1\"", exe))?;
        Ok(())
    }
}

pub fn register_nxm_handler() -> io::Result<()> {
    platform::register()
}

