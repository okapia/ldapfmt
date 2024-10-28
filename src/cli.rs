use clap::{Command, arg, command, crate_version, crate_description, value_parser, ValueHint};

#[derive(clap::ValueEnum, Clone, Debug, Default, PartialEq)]
pub enum SaslMech {
    #[default]
    Simple,
    External,
    Gssapi,
}

pub fn build_cli() -> Command {
    command!().version(crate_version!())
        .about(crate_description!())
        .arg(arg!(uri: -H <URI> "specify the LDAP server URI").value_hint(ValueHint::Url).value_parser(value_parser!(String)),)
        .arg(arg!(base: -b <BASE> "specify the search base (Base DN)").value_hint(ValueHint::Other).value_parser(value_parser!(String)),)
        .arg(arg!(<TEMPLATE> "Jinja2 template file").value_hint(ValueHint::FilePath).value_parser(value_parser!(String)).required(true),)
        .arg(arg!([ARGS]... "arguments to template").value_hint(ValueHint::AnyPath).value_parser(value_parser!(String)),)
        .arg(arg!(binddn: -D <BIND_DN> "specify the Bind DN for authentication").value_hint(ValueHint::Other).value_parser(value_parser!(String)),)
        .arg(arg!(password: -w <PASSWORD> "specify the password for simple authentication").value_hint(ValueHint::Other).value_parser(value_parser!(String)),)
        .arg(arg!(saslmech: -Y <MECH> "specify the SASL mechanism to be used for authentication").value_parser(value_parser!(SaslMech)))
        .arg(arg!(starttls: -Z  "use StartTLS to establish a secure connection"))
}
