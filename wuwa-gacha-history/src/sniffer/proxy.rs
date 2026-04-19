pub struct ProxyGuard {
    inner: Inner,
}

impl ProxyGuard {
    pub async fn enable(port: u16) -> Result<Self, String> {
        let inner = Inner::enable(port).await?;
        Ok(Self { inner })
    }

    pub async fn disable(self) -> Result<(), String> {
        self.inner.disable().await
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use winreg::RegKey;
    use winreg::enums::*;

    const KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

    pub struct Inner {
        prev_enable: u32,
        prev_server: Option<String>,
        prev_override: Option<String>,
    }

    impl Inner {
        pub async fn enable(port: u16) -> Result<Self, String> {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (key, _) = hkcu.create_subkey(KEY).map_err(|e| e.to_string())?;

            let prev_enable: u32 = key.get_value("ProxyEnable").unwrap_or(0);
            let prev_server: Option<String> = key.get_value("ProxyServer").ok();
            let prev_override: Option<String> = key.get_value("ProxyOverride").ok();

            key.set_value("ProxyEnable", &1u32)
                .map_err(|e| e.to_string())?;
            key.set_value("ProxyServer", &format!("127.0.0.1:{port}"))
                .map_err(|e| e.to_string())?;
            key.set_value("ProxyOverride", &"<local>".to_string())
                .map_err(|e| e.to_string())?;

            notify_wininet();

            Ok(Self {
                prev_enable,
                prev_server,
                prev_override,
            })
        }

        pub async fn disable(self) -> Result<(), String> {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (key, _) = hkcu.create_subkey(KEY).map_err(|e| e.to_string())?;
            key.set_value("ProxyEnable", &self.prev_enable)
                .map_err(|e| e.to_string())?;
            match self.prev_server {
                Some(s) => key
                    .set_value("ProxyServer", &s)
                    .map_err(|e| e.to_string())?,
                None => {
                    let _ = key.delete_value("ProxyServer");
                }
            }
            match self.prev_override {
                Some(s) => key
                    .set_value("ProxyOverride", &s)
                    .map_err(|e| e.to_string())?,
                None => {
                    let _ = key.delete_value("ProxyOverride");
                }
            }
            notify_wininet();
            Ok(())
        }
    }

    fn notify_wininet() {
        use windows::Win32::Networking::WinInet::{
            INTERNET_OPTION_REFRESH, INTERNET_OPTION_SETTINGS_CHANGED, InternetSetOptionW,
        };
        unsafe {
            let _ = InternetSetOptionW(None, INTERNET_OPTION_SETTINGS_CHANGED, None, 0);
            let _ = InternetSetOptionW(None, INTERNET_OPTION_REFRESH, None, 0);
        }
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use tokio::process::Command;

    pub struct Inner {
        services: Vec<String>,
    }

    impl Inner {
        pub async fn enable(port: u16) -> Result<Self, String> {
            let services = list_services().await?;
            if services.is_empty() {
                return Err("no active network services found".into());
            }
            let port_s = port.to_string();
            let mut script = String::from("do shell script \"");
            for svc in &services {
                let svc_q = shell_escape(svc);
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setwebproxy {svc_q} 127.0.0.1 {port_s} && "
                ));
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setsecurewebproxy {svc_q} 127.0.0.1 {port_s} && "
                ));
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setwebproxystate {svc_q} on && "
                ));
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setsecurewebproxystate {svc_q} on && "
                ));
            }
            script.push_str("true\" with administrator privileges");

            let status = Command::new("osascript")
                .args(["-e", &script])
                .status()
                .await
                .map_err(|e| e.to_string())?;
            if !status.success() {
                return Err("failed to enable system proxy (user may have cancelled)".into());
            }
            Ok(Self { services })
        }

        pub async fn disable(self) -> Result<(), String> {
            let mut script = String::from("do shell script \"");
            for svc in &self.services {
                let svc_q = shell_escape(svc);
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setwebproxystate {svc_q} off && "
                ));
                script.push_str(&format!(
                    "/usr/sbin/networksetup -setsecurewebproxystate {svc_q} off && "
                ));
            }
            script.push_str("true\" with administrator privileges");

            let status = Command::new("osascript")
                .args(["-e", &script])
                .status()
                .await
                .map_err(|e| e.to_string())?;
            if !status.success() {
                return Err("failed to disable system proxy".into());
            }
            Ok(())
        }
    }

    async fn list_services() -> Result<Vec<String>, String> {
        let out = Command::new("networksetup")
            .arg("-listallnetworkservices")
            .output()
            .await
            .map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&out.stdout);
        Ok(text
            .lines()
            .skip(1)
            .filter(|l| !l.starts_with('*') && !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .collect())
    }

    fn shell_escape(s: &str) -> String {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod platform {
    pub struct Inner;

    impl Inner {
        pub async fn enable(_port: u16) -> Result<Self, String> {
            Err("system proxy not implemented on this platform".into())
        }
        pub async fn disable(self) -> Result<(), String> {
            Ok(())
        }
    }
}

use platform::Inner;
