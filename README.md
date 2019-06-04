# DGRESSIONS

* It must mount the vital kernel filesystem `/dev,/sys,/var,/proc`.
* It must mount the root partition, first in read only, check the partition integrity `fsck` and the remount-it with write privileges.
* It must mount user-defined partitions by reading `/etc/fstab`
* It must load required kernel modules, and user-defined ones. See `udev`
* It must start and use the syslog
* It must activate swap
* It must init the Linux consoles `TTYs`
* It must configure and start user-defined network interfaces
* It must start, and watch user-defined daemons
* Mounted encrypted partitions `crypfs`
* Early-start in RAM only filesystem `initramfs`
* Setup System Clock
* Start X and X environment
* Start the cron daemon
* Init system locales
* Init system hostname
* Anything that systemd does that you think is cool

# GRESSIONS

* See the status of a daemon
* Start a deamon
* Stop a daemon
* Reload the configuration without stopping the program
* Enable or Disable a daemon (Do / Do not start it by default)
