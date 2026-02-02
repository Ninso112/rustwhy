//! Permission and capability checking for diagnostic modules.

use crate::core::traits::Permission;
use std::path::Path;

/// Check if the current process has root (effective UID 0).
pub fn is_root() -> bool {
    users::get_effective_uid() == 0
}

/// Check if we can read from /proc (e.g. /proc/self/status).
pub fn can_read_proc() -> bool {
    Path::new("/proc/self/status").exists()
        && std::fs::read_to_string("/proc/self/status").is_ok()
}

/// Check if we can read from /sys (e.g. /sys/class).
pub fn can_read_sys() -> bool {
    Path::new("/sys/class").exists() && std::fs::read_dir("/sys/class").is_ok()
}

/// Check if the given permission is satisfied on this system.
pub fn has_permission(perm: &Permission) -> bool {
    match perm {
        Permission::Root => is_root(),
        Permission::ReadProc => can_read_proc(),
        Permission::ReadSys => can_read_sys(),
        Permission::NetAdmin => is_root(), // Simplified; could check CAP_NET_ADMIN
        Permission::PerfEvent => is_root(), // Simplified; could check CAP_SYS_ADMIN / perf_paranoid
    }
}

/// Check if all given permissions are satisfied.
pub fn has_all_permissions(permissions: &[Permission]) -> bool {
    permissions.iter().all(has_permission)
}
