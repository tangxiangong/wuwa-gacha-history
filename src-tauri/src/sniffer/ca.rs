use std::path::{Path, PathBuf};

use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DistinguishedName, DnType, IsCa, KeyPair,
    KeyUsagePurpose,
};
use time::{Duration, OffsetDateTime};
use tokio::fs;

const CA_COMMON_NAME: &str = "wuwa-gacha-history MITM CA";
const CA_CERT_FILE: &str = "ca.pem";
const CA_KEY_FILE: &str = "ca.key.pem";

pub struct CaMaterial {
    cert_pem: String,
    key_pem: String,
    pub cert_path: PathBuf,
}

impl CaMaterial {
    pub async fn load_or_generate(dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(dir).await.map_err(|e| e.to_string())?;
        let cert_path = dir.join(CA_CERT_FILE);
        let key_path = dir.join(CA_KEY_FILE);

        if cert_path.exists() && key_path.exists() {
            let cert_pem = fs::read_to_string(&cert_path)
                .await
                .map_err(|e| e.to_string())?;
            let key_pem = fs::read_to_string(&key_path)
                .await
                .map_err(|e| e.to_string())?;
            return Ok(Self { cert_pem, key_pem, cert_path });
        }

        let (cert_pem, key_pem) = generate_ca()?;
        fs::write(&cert_path, &cert_pem)
            .await
            .map_err(|e| e.to_string())?;
        fs::write(&key_path, &key_pem)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Self { cert_pem, key_pem, cert_path })
    }

    pub fn into_key_and_cert(self) -> Result<(KeyPair, Certificate), String> {
        let key = KeyPair::from_pem(&self.key_pem).map_err(|e| e.to_string())?;
        let cert = CertificateParams::from_ca_cert_pem(&self.cert_pem)
            .map_err(|e| e.to_string())?
            .self_signed(&key)
            .map_err(|e| e.to_string())?;
        Ok((key, cert))
    }
}

fn generate_ca() -> Result<(String, String), String> {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
        KeyUsagePurpose::DigitalSignature,
    ];
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, CA_COMMON_NAME);
    dn.push(DnType::OrganizationName, "wuwa-gacha-history");
    params.distinguished_name = dn;
    let now = OffsetDateTime::now_utc();
    params.not_before = now - Duration::days(1);
    params.not_after = now + Duration::days(3650);

    let key = KeyPair::generate().map_err(|e| e.to_string())?;
    let cert = params.self_signed(&key).map_err(|e| e.to_string())?;
    Ok((cert.pem(), key.serialize_pem()))
}

#[cfg(target_os = "macos")]
pub async fn install_to_system_trust(cert_path: &Path) -> Result<(), String> {
    use tokio::process::Command;
    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    let keychain = format!("{home}/Library/Keychains/login.keychain-db");
    let cert = cert_path.to_string_lossy().to_string();
    let output = Command::new("security")
        .args([
            "add-trusted-cert",
            "-r",
            "trustAsRoot",
            "-k",
            &keychain,
            &cert,
        ])
        .output()
        .await
        .map_err(|e| format!("无法调用 security: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stderr}{stdout}").trim().to_string();

    if combined.contains("already") && combined.contains("trust") {
        return Ok(());
    }

    let _ = Command::new("open")
        .args(["-a", "Keychain Access", &cert])
        .status()
        .await;

    Err(format!(
        "自动安装证书失败：{combined}\n已用钥匙串访问打开证书文件，请双击该证书并在「信任」里将 \"Secure Sockets Layer (SSL)\" 改为「始终信任」，然后重新点击抓包。"
    ))
}

#[cfg(target_os = "windows")]
pub async fn install_to_system_trust(cert_path: &Path) -> Result<(), String> {
    use tokio::process::Command;
    let cert = cert_path.to_string_lossy().to_string();
    let output = Command::new("certutil")
        .args(["-user", "-addstore", "Root", &cert])
        .output()
        .await
        .map_err(|e| format!("无法调用 certutil: {e}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(format!(
        "certutil -addstore Root 失败：{}{}",
        stderr.trim(),
        stdout.trim()
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub async fn install_to_system_trust(_cert_path: &Path) -> Result<(), String> {
    Err("CA install not implemented on this platform".into())
}
