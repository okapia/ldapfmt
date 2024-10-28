use crate::cli::SaslMech;
use std::path::Path;
use url::Url;

#[derive(Debug, Default)]
pub struct ConfigSettings {
    pub uri: Vec<Url>,
    pub base: String,
    pub binddn: String,
    pub password: String,
    pub sasl_mech: SaslMech,
    pub starttls: bool,
}

pub fn parse_config(mut cfg: ConfigSettings) -> Result<ConfigSettings, anyhow::Error> {
    let mut ldap_conf_path = Path::new("/etc/openldap/ldap.conf");
    if !ldap_conf_path.exists() {
        ldap_conf_path = Path::new("/etc/ldap/ldap.conf");
    }
    if ldap_conf_path.exists() {
        for line in std::fs::read_to_string(ldap_conf_path)?.lines() {
            if let Some(end) = line.find(' ') {
                match &line[0..end] {
                    "BASE" => {
                        if cfg.base.is_empty() {
                            cfg.base = line[end + 1..].trim().to_string();
                        }
                    }
                    "BINDDN" => {
                        if cfg.binddn.is_empty() {
                            cfg.binddn = line[end + 1..].trim().to_string();
                        }
                    }
                    "SASL_MECH" => {
                        cfg.sasl_mech = match line[end + 1..].trim() {
                            "GSSAPI" => SaslMech::Gssapi,
                            "EXTERNAL" => SaslMech::External,
                            _ => SaslMech::Simple,
                        }
                    }
                    "URI" => {
                        let matches: Vec<_> = line[end + 1..]
                            .trim()
                            .split(' ')
                            .filter(|e| !e.is_empty())
                            .filter_map(|u| Url::parse(u).ok())
                            .collect();
                        if !matches.is_empty() {
                            cfg.uri = matches;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(cfg)
}
