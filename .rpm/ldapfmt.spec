%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: ldapfmt
Summary: Template-Driven Formatted Output from LDAP Queries
Version: @@VERSION@@
Release: 1
License: MIT or ASL 2.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
BuildRequires: libselinux-devel
BuildRequires: selinux-policy-devel
URL: https://github.com/okapia/ldapfmt

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
Ldapfmt combines the jinja2 templating language with LDAP search functionality
to allow data from LDAP to be formatted in a variety of different manners.

%prep
%setup -q

%build
make -C ../../../../../selinux -f /usr/share/selinux/devel/Makefile ldapfmt.pp

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}
mkdir -m 0755 -p %{buildroot}%{_datadir}/zsh/site-functions
mkdir -m 0755 -p %{buildroot}%{_datadir}/bash-completion/completions
mkdir -m 0755 -p %{buildroot}%{_datadir}/selinux/packages
mkdir -m 0755 -p %{buildroot}%{_mandir}/man1
cp ../../../../../doc/ldapfmt.1 %{buildroot}%{_mandir}/man1
chmod 644 %{buildroot}%{_mandir}/man1/ldapfmt.1
cp ../../../../../_ldapfmt %{buildroot}%{_datadir}/zsh/site-functions
chmod 644 %{buildroot}%{_datadir}/zsh/site-functions/_ldapfmt
cp ../../../../../ldapfmt.bash %{buildroot}%{_datadir}/bash-completion/completions/ldapfmt
chmod 644 %{buildroot}%{_datadir}/bash-completion/completions/ldapfmt
cp ../../../../../selinux/ldapfmt.pp %{buildroot}%{_datadir}/selinux/packages
chmod 644 %{buildroot}%{_datadir}/selinux/packages/ldapfmt.pp

%clean
rm -rf %{buildroot}

%pre
%selinux_relabel_pre

%post
%selinux_modules_install %{_datadir}/selinux/packages/ldapfmt.pp
%selinux_relabel_post

%posttrans
%selinux_relabel_post

%postun
%selinux_modules_uninstall ldapfmt
if [ $1 -eq 0 ]; then
    %selinux_relabel_post
fi

%files
%defattr(-,root,root,-)
%{_bindir}/*
%{_mandir}/
%{_datadir}/
