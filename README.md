A simple script to generate RPMs from sddm theme tar files from pling.

## Importance
Fedora Distributions using OSTree use a read-only system image, that is downloaded and can only be modified via `rpm-ostree override`.
Currently this command can only remove or add RPM packages, not change, remove or add system files manually.

What this tool does, is creating an RPM thats only content is the SDDM theme, placed at the correct location. 

## Usage in Fedora OSTree (Silverblue, Kinoite,...)
Use Toolbox or Distrobox for compiling this app, as it needs some dependencies that would otherwise need to be layered.

### 1. Container
Toolbox:
```
toolbox create fedora && toolbox enter fedora
```

Distrobox:
```
distrobox create fedora && distrobox enter fedora
```

### 2. Dependencies, Downloading
```
sudo dnf upgrade -y
sudo dnf install cargo rustc git

git clone https://github.com/Lunarequest/sddm2rpm

cargo build --path ~/sddm2rpm

```

### 3. Add sddm2rpm to $PATH
This has the effect that you can run it directly from the Terminal, without entering the path manually.

print path:
```
echo $PATH
```

add Cargo to Path:
```
printf """
export PATH=/var/home/user/.cargo/bin/:$PATH""" >> ~/.bashrc
```

### 4. Use the tool
- Go to pling.com
- You can use this search engine: https://www.pling.com/find?search=%s
- Download any SDDM theme
- if not in .tar.gz format, unpack the folder and repack it as .tar.gz (using Ark for example)

Convert the archive to an RPM:
```
sddm2rpm path/to/sddm-theme.tar.gz --version=1.0
```

***It's important to add the version, otherwise the RPM can't be installed.***

### 5. Layer the rpm
In Dolphin for example, you can copy the address of the rpm.
```
rpm-ostree install /path/to/sddm.rpm
reboot
```

After rebooting, the SDDM design will appear in your settings!

---

I will include lots of prebuilt RPM packages, so you dont have to do it yourself.

This tool has potential to be used for other `rpm-ostree` overrides like cursors and settings too.
