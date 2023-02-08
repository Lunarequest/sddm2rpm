A simple script to generate RPMs from sddm theme tar files from pling.

## Importance
Fedora Distributions using OSTree as well as OpenSuse microOS have a read-only system image that is downloaded and can only be modified via `rpm-ostree override` / `transactional-updateo` n your device (without actually hosting your own repository).

Currently these commands can only remove or add RPM packages, not change, remove or add system files manually, which limits the customizability of these Distributions a lot, even though OSTree is a perfect solution for a solid system, automatic updates, e.g.

What this tool does is creating an RPM whose only content is the SDDM theme, placed at the correct location. 


## Usage in microOS or Fedora OSTree (Silverblue, Kinoite,...)
Use Toolbox or Distrobox for compiling this app. (Fedora has Toolbox preinstalled, microOS uses the more flexible Distrobox)

### 1. Container
Toolbox
```
toolbox create build && toolbox enter build
```

Distrobox:
```
distrobox create build && distrobox enter build
```


## 2.1 Easy installation
You can skip building the program yourself and right away use the prebuilt binary:

```
cargo add sddm2rpm
```

### 2.2 Build it yourself
Fedora: 
```
sudo dnf install -y cargo git printf

git clone https://github.com/Lunarequest/sddm2rpm

cargo build --path ~/sddm2rpm/
```

microOS:
```
sudo zypper in -y cargo git printf

git clone https://github.com/Lunarequest/sddm2rpm

cargo build --path ~/sddm2rpm/
```

### 3. Add sddm2rpm (cargo) to $PATH
This has the effect that you can run it directly from the Terminal, without entering the path manually.

add Cargo to Path:
```
printf """
export PATH=/var/home/user/.cargo/bin/:$PATH""" >> ~/.bashrc
```

print cargo path to check:
```
echo $PATH | grep .cargo
```

### 4. Use the tool
- Go to pling.com
- You can use this search engine: https://www.pling.com/find?search=%s
- Download any SDDM theme
- if its a .zip, unpack the folder and repack it as .tar.gz or another format (using Ark for example)

Convert the archive to an RPM:
```
sddm2rpm path/to/sddm-theme.tar.gz
```

### 5. Layer the rpm
In Dolphin for example, you can copy the address of the rpm.

Fedora:
```
rpm-ostree install /path/to/sddm.rpm
reboot
```

microOS: 
```
transactional-update pkg in path/to/sddm.rpm
reboot
```

After rebooting, the SDDM design will appear in your settings!

---

*Comment: Due to update and ownership problems, there won't be any prebuilt RPMs on this Repository.*
