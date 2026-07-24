Name:           cosmic-osd
Epoch:          1
Version: 1.2.0
Release:        1%{?dist}
Summary:        COSMIC OSD and polkit agent (Playtron fork)

License:        GPL-3.0-only
URL:            https://github.com/pop-os/cosmic-osd
Source0:        %{name}-%{_arch}.tar.gz

# Binary is pre-built (see Taskfile dist:prepare); no debuginfo extraction.
%global debug_package %{nil}

# No BuildRequires - binary is pre-built

# Runtime deps mirror stock cosmic-osd. polkit provides polkit-agent-helper-1
# (the PAM helper this agent execs); pulseaudio-utils backs the audio OSD;
# cosmic-icon-theme is the noarch icon set — bind to COSMIC 1.x (< 2.0.0).
Requires:       polkit
Requires:       dbus
Requires:       pulseaudio-utils
Requires:       (cosmic-icon-theme >= 1.0.0 with cosmic-icon-theme < 2.0.0)

# Override the upstream/stock cosmic-osd. Forked packages carry Epoch:1 so their
# constraints outrank the stock (epoch-less) Fedora package.
Provides:       cosmic-osd = %{epoch}:%{version}-%{release}
Obsoletes:      cosmic-osd < %{epoch}:%{version}

%description
On-screen display and polkit authentication agent for the COSMIC desktop
environment. Renders volume/brightness/media OSDs and presents polkit password
prompts to the user.

%prep
%autosetup -n %{name} -p1

%build

%install
install -Dm0755 "usr/bin/cosmic-osd" "%{buildroot}%{_bindir}/cosmic-osd"
install -Dm0644 "usr/share/licenses/cosmic-osd/LICENSE" "%{buildroot}%{_datadir}/licenses/cosmic-osd/LICENSE"

%files
%license %{_datadir}/licenses/cosmic-osd/LICENSE
%{_bindir}/cosmic-osd

%changelog
* Fri Jul 24 2026 Playtron <dev@playtron.one> - 1.2.0-1
- Initial RPM package (Playtron fork)
- Resolve polkit-agent-helper-1 across distro paths (fixes auth prompts on Fedora)
