extern crate clap;

use crate::ldapconf::ConfigSettings;
use crate::cli::{build_cli,SaslMech};
use anyhow::{Context, Result};
use ldap3::{LdapConn, LdapConnSettings, Scope, SearchEntry};
use minijinja::{context, path_loader, Environment, Error, ErrorKind};
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use url::Url;

mod cli;
mod ldapconf;

fn args() -> (ConfigSettings, String, Arc<Vec<String>>) {
    let mut settings = ConfigSettings::default();

    let matches = build_cli().get_matches();

    // Get and process the arguments
    if let Some(uri) = matches.get_one::<String>("uri") {
        settings.uri = uri
            .trim()
            .split(&[' ',','][..])
            .filter(|e| !e.is_empty())
            .filter_map(|u| Url::parse(u).ok()) // errors here do matter
            .collect();
    }

    if let Some(base_dn) = matches.get_one::<String>("base") {
        settings.base = base_dn.trim().to_string();
    }
    if let Some(bind_dn) = matches.get_one::<String>("binddn") {
        settings.binddn = bind_dn.trim().to_string();
    }
    if let Some(password) = matches.get_one::<String>("password") {
        settings.password = password.to_string();
    }
    if let Some(saslmech) = matches.get_one::<SaslMech>("saslmech") {
        settings.sasl_mech = saslmech.clone();
    }
    settings.starttls = matches.get_flag("starttls");

    let args = if let Some(a) = matches.get_many::<String>("ARGS") {
        a.map(|x| x.to_owned()).collect()
    } else {
        Vec::with_capacity(0)
    };
    (
        settings,
        matches.get_one::<String>("TEMPLATE").unwrap().clone(),
        Arc::new(args),
    )
}

fn main() -> Result<(), anyhow::Error> {
    let (mut data, template, args) = args();
    let path = std::path::Path::new(template.as_str());
    let prefix = path
        .parent()
        .context("Can't get parent directory of input file")?;
    let stdin =
        fs::read_to_string(path).context(format!("{}: Failed to read input", path.display()))?;
    let template = path.to_str().unwrap_or("input");
    if data.uri.is_empty() {
        data = ldapconf::parse_config(data)?;
    }
    let pos = if &stdin[0..2] == "#!" {
        stdin.find('\n').map_or(0, |i| i + 1)
    } else {
        0
    };
    let mut i = data.uri.iter();
    let settings = LdapConnSettings::new().set_conn_timeout(Duration::from_millis(500)).set_starttls(data.starttls);
    let ldap = Arc::new(Mutex::new(loop {
        if let Some(url) = i.next() {
            if let (Ok(mut l), Some(host)) = (
                LdapConn::from_url_with_settings(settings.clone(), url),
                url.host_str(),
            ) {
                if data.sasl_mech == SaslMech::Gssapi {
                    l.sasl_gssapi_bind(host)?;
                } else if data.sasl_mech == SaslMech::External {
                    l.sasl_external_bind()?;
                } else if !data.binddn.is_empty() {
                    l.simple_bind(data.binddn.as_str(), data.password.as_str())?;
                }
                break l;
            }
        } else {
            eprintln!("Unable to find usable LDAP server");
            exit(1);
        }
    }));
    let mut env = Environment::new();
    env.set_loader(path_loader(prefix));
    env.add_template(template, &stdin[pos..])
        .context("Failed to compile template")?;
    let closureldap = Arc::clone(&ldap);
    env.add_function(
        "search",
        move |filter: String,
              fields: Vec<String>|
              -> Result<Vec<HashMap<String, Vec<String>>>, Error> {
            let mut l = closureldap
                .lock()
                .map_err(|e| Error::new(ErrorKind::InvalidOperation, e.to_string()))?;
            let (rs, _res) = l
                .search(&data.base, Scope::Subtree, filter.as_str(), fields)
                .map_err(|e| Error::new(ErrorKind::InvalidOperation, e.to_string()))?
                .success()
                .map_err(|e| Error::new(ErrorKind::InvalidOperation, e.to_string()))?;
            Ok(rs
                .iter()
                .map(|entry| {
                    // can the clone be avoided by consuming the vector
                    let mut s = SearchEntry::construct(entry.clone());
                    s.attrs.insert("dn".to_string(), vec![s.dn]);
                    s.attrs
                })
                .collect::<Vec<_>>())
        },
    );
    env.add_function("args", move || -> Vec<String> { args.to_vec() });
    env.add_function("error", |msg: String| -> Result<(), Error> {
        Err(Error::new(ErrorKind::InvalidOperation, msg))
    });
    let tmpl = env.get_template(template)?;
    println!("{}", tmpl.render(context!())?);
    if let Ok(mut l) = ldap.lock() {
        l.unbind()?;
    }
    Ok(())
}
