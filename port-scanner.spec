Name:           port-scanner_rust
Version:        1.0.1
Release:        1%{?dist}
Summary:        A simple and stylish TCP port scanner written in Rust

License:        MIT
URL:            https://github.com/0xNejiHyuga/port-scanner_rust
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildRequires:  rust-packaging
# Эти макросы автоматически подтянут зависимости из Cargo.toml
%cargo_generate_buildrequires

%description
A fast and simple TCP port scanner written in Rust.
Features a stylish ASCII banner and colored output for open ports.

%prep
%autosetup -n %{name}-%{version}

%build
%cargo_build

%install
%cargo_install

%files
%{_bindir}/port-scanner_rust

%changelog
* Sat Nov 07 2026 0xNejiHyuga <your-email@example.com> - 1.0.0-1
- Initial package release