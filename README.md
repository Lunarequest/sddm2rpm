A simple script to genrate rpms from sddm theme tar files from pling.

## Background
When using this one a read-only distro like OpenSUSE MicroOS or Fedora Kinonite, you can't install sddm theme's through the KCM as `/usr/share` is readonly. My solution for this was `sddm2rpm` a tool that reads the archives used by KDE plasma and the pling store to produce a working rpm that allows the sddm theme to be install and configured in the KCM.

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

If you want to host rpms produced by this project I recomend using the `--output-spec` option to produce a working spec file you can use to produce rpms in [open build service](https://build.opensuse.org) & [copr](https://copr.fedorainfracloud.org).

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
