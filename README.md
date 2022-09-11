ragent is my Rust learning project (so its implementation sucks; PRs, issues and suggestions welcome).

ragent is my own monitoring agent:

* It works with Nagios (with check_ragent).
* It is as simple and lightweight as possible.

# DISCLAIMER

* I am a complete noob on Rust.
* I have no idea about packaging. The rpm and deb are TERRIBLE.

# USAGE

```
$ ragent
```

will start ragent listening at *:21488.

```
$ check_ragent http://host:21488/
```

will contact a ragent instance at http://host:21488/ and generate output/perfdata/return code following Nagios guidelines.

```
$ check_ragent
```

will check the local host without using a daemon.

# WHAT'S MONITORED

* No filesystem's free space is less than 2 GB or 20% free (warning) or less than 1 GB/10% (critical)
* No filesystem's free inodes are less than 20% free (warning) or less than 10% free (critical)
* No SystemD unit is in failed state (critical, or use `--warning-units` to define units that will only generate warnings)
* No reboot is required (EL7, EL8, Debian/Ubuntu)
  (Note that Rocky Linux 8 on Raspberry Pi seems to be affected by [this bug that requires manual intervention after reboots](https://bugs.rockylinux.org/show_bug.cgi?id=177))
* Entropy is over a quarter of the pool size (critical) or over half (warning) 

# BUILDING PACKAGES

See [README](packages/README) on packages directory.

Packages are available at:

https://cloudsmith.io/~ragent/repos/ragent/setup/

Packages are tested on EL7, EL8 (x86 and aarch64), Debian 10, Debian 11, and Ubuntu 20.04.

# MAKING RELEASES

Run `cargo release --no-tag major|minor|patch` on a branch.
Merge the branch.
Tag the version commit and push the tag.
