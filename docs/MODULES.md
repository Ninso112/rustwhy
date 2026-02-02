# Modules

Each module answers a "why" question about the system.

| Module | Subcommand | Purpose |
|--------|------------|---------|
| boot   | `rustwhy boot`   | Why is boot slow? (systemd-analyze, blame, critical-chain) |
| cpu    | `rustwhy cpu`   | Why is CPU busy? (load, top processes) |
| mem    | `rustwhy mem`   | Why is memory full? (/proc/meminfo, top processes) |
| disk   | `rustwhy disk`  | Why is disk full? (directory sizes, large/old files) |
| io     | `rustwhy io`    | Why is disk I/O high? (/proc/diskstats, per-process I/O) |
| net    | `rustwhy net`   | Why is network slow? (ping, DNS, interfaces) |
| fan    | `rustwhy fan`   | Why are fans spinning? (hwmon, correlation with temp) |
| temp   | `rustwhy temp`  | Why is system hot? (thermal zones, throttling) |
| gpu    | `rustwhy gpu`   | Why is GPU busy/idle? (NVIDIA/AMD/Intel) |
| batt   | `rustwhy batt`  | Why is battery draining? (power_supply, wakeups) |
| sleep  | `rustwhy sleep` | Why won't it sleep? (inhibitors, wake sources) |
| usb    | `rustwhy usb`   | Why isn't USB working? (device tree, dmesg) |
| mount  | `rustwhy mount`| Why is mount failing? (/proc/mounts, fstab, NFS) |

Run `rustwhy <subcommand> --help` for module-specific options.
