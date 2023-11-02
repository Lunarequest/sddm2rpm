A simple script to genrate RPMs from SDDM theme Archives from pling/store.kde.org.

## Background
When using this one a read-only distro like OpenSUSE Kalpa or Fedora Kinoite, you can't install SDDM themes through the KCM, as `/usr/share` is read-only. My solution for this was `sddm2rpm`, a tool that reads the archives used by KDE plasma to produce a working RPM that allows the SDDM theme to be installed and configured in the KCM.

## Building 

I recommend using toolbox for building sddm2rpm as it is preinstalled on OpenSUSE MicroOS and Fedora Kinoite. 

### Dependencies 

You will need rust as well as gcc or clang. 

```
cargo install --path ~/sddm2rpm/
```
or 
```
cargo install sddm2rpm
```
for stable releases. 

If you want to host RPMs produced by this project I recommend using the `--output-spec` option to produce a working spec file. You can use it to produce rpms in [Open Build Service](https://build.opensuse.org) & [COPR](https://copr.fedorainfracloud.org).

## Arguments

```
takes sddm theme as tar.gz files and repacks them to rpms

Usage: sddm2rpm [OPTIONS] <source> [dest]

Arguments:
  <source>  path to sddm archive
  [dest]    directory to unpack too

Options:
      --pkg-version <VERSION>  version of package, defaults to 1.0.0
      --license <LICENSE>      license of package, defaults to GPLv3
  -s, --output-spec            output spec file
      --url <URL>              upstream url for rpm
  -h, --help                   Print help
  -V, --version                Print version

```
