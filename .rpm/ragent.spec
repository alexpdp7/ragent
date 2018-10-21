%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: ragent
Summary: A Rust minimal monitoring agent
Version: @@VERSION@@
Release: 1
License: Public Domain
Group: System Environment/Daemons
Source0: %{name}-%{version}.tar.gz

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root
BuildRequires: systemd

Requires(pre): shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%systemd_post ragent.service

%preun
%systemd_preun ragent.service

%postun
%systemd_postun_with_restart ragent.service

%files
%defattr(-,root,root,-)
%{_bindir}/*
%{_unitdir}/ragent.service

%pre
getent group ragent >/dev/null || groupadd -r ragent
getent passwd ragent >/dev/null || \
    useradd -r -g ragent -d / -s /sbin/nologin ragent
exit 0
