# ldapfmt

A tool for running LDAP queries and formatting the results using jinja2.

When using LDAP, writing a shell script which uses `ldapsearch`, typically
piped to `sed`, ends up being the clumsy solution to a variety of problems. If
multiple searches are needed, this is not especially efficient because separate
connections to the LDAP server are made. There are scripting languages that can
hook into a proper API for LDAP queries but for typical use cases, a template
language can express the desired results more succinctly.

ldapfmt combines [minijinja](https://github.com/mitsuhiko/minijinja) with
[ldap3](https://github.com/inejge/ldap3). To use, it either pass the path to a
template as a parameter or simply use a Unix shebang line and treat the
template as a script. A shebang line be skipped so that it doesn't appear in
the output but note that line numbers from minijinja don't account for this.

For LDAP connection parameters, `/etc/openldap/ldap.conf` is parsed.

## Functions

The following additional Jinja2 functions are defined:

`search(filter, [ fields, ... ])`
: Return the result of an LDAP search. `filter` is a standard LDAP filter
expression. Results are limited to the specified fields. These are all lists
because there can be multiple values with the same key in an LDAP entry.

`error(message)`
: Abort processing returning status 1 and the specfied error message.

## Variables

The following additional Jinja2 variables are used:

`args`
: Contains the command-line arguments

`searchbase`
: Contains the base dn used for searches

## Examples

The `sample` directory contains a number of examples:

`ldap_ssh_authorizedkeys`
: An example for use with the `AuthorizedKeysCommand` of `sshd` which returns
ssh public keys for a user, also checking `sudo` rules for shared-role accounts
to add public keys for additional users.

`authzsvn`
: Expands groups in a configuration file for a subversion server.

`permissions_wiki`
: Dumps groups from [FreeIPA](https://www.freeipa.org/) where management of the
group had been delegated. Output in a [MediaWiki](https://www.mediawiki.org/)
format table.
