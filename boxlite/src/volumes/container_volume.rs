//! Container-level volume management.
//!
//! Manages bind mounts for the container layer.

use std::path::PathBuf;

use super::guest_volume::GuestVolumeManager;

/// Container bind mount entry.
#[derive(Debug, Clone)]
pub struct ContainerMount {
    /// Source path in guest VM
    pub source: String,
    /// Destination path in container
    pub destination: String,
    /// Read-only mount
    pub read_only: bool,
}

/// Manages container-level volume configuration.
///
/// Tracks bind mounts from guest VM paths into container namespace.
pub struct ContainerVolumeManager {
    container_mounts: Vec<ContainerMount>,
}

impl ContainerVolumeManager {
    /// Create a new container volume manager.
    pub fn new() -> Self {
        Self {
            container_mounts: Vec::new(),
        }
    }

    /// Add a volume visible to both guest and container.
    ///
    /// Sets up virtiofs share in guest, records bind mount for container.
    pub fn add_volume(
        &mut self,
        guest: &mut GuestVolumeManager,
        host_path: PathBuf,
        guest_path: &str,
        container_path: &str,
        read_only: bool,
    ) {
        // Add virtiofs share to guest
        let tag = guest.next_auto_tag();
        guest.add_fs_share(&tag, host_path, guest_path, read_only);

        // Record container bind mount
        self.container_mounts.push(ContainerMount {
            source: guest_path.to_string(),
            destination: container_path.to_string(),
            read_only,
        });
    }

    /// Add a container bind mount directly.
    ///
    /// Use when guest path already exists (e.g., from block device mount).
    pub fn add_bind(&mut self, guest_path: &str, container_path: &str, read_only: bool) {
        self.container_mounts.push(ContainerMount {
            source: guest_path.to_string(),
            destination: container_path.to_string(),
            read_only,
        });
    }

    /// Build container mount configuration.
    pub fn build_container_mounts(&self) -> Vec<ContainerMount> {
        self.container_mounts.clone()
    }
}

impl Default for ContainerVolumeManager {
    fn default() -> Self {
        Self::new()
    }
}
