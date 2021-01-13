use crate::KERNEL;
use crate::{KernelVersion, ProcError, ProcResult};

use std::{fs::read_link, path::PathBuf};

macro_rules! since_kernel {
    ($a:tt, $b:tt, $c:tt, $e:expr) => {
        if *KERNEL >= KernelVersion::new($a, $b, $c) {
            $e
        } else {
            Ok(None)
        }
    };
}

/// Namesapce information about the process, based on the `/proc/<pid>/ns/*` file.
/// To construct one of these structures, your have to first create a [Process](crate::process::Process).
///
/// Not all fields are available in every kernel.  These fields have `Options<T>` types.
///

/// According to manual page namespaces(7), /proc/$$/ns files were visible as hard links,
/// since Linux 3.8, they appear as stmbilic links.
#[derive(Debug, Clone, Default)]
pub struct Namespaces {
    _private: (),
    /// since Linux 4.6
    pub cgroup: Option<String>,
    /// since Linux 3.0
    pub ipc: Option<String>,
    /// since Lnux 3.8
    pub mnt: Option<String>,
    /// since Linux 3.8
    pub net: Option<String>,
    /// since Linux 3.8
    pub pid: Option<String>,
    /// since Linux 3.8
    pub user: Option<String>,
    /// since Linux 3.0
    pub uts: Option<String>,
    /// since Linx 4.12
    pub pid_for_children: Option<String>,
    /// since Linux 5.6
    pub time: Option<String>,
    /// since Linux 5.6
    pub time_for_children: Option<String>,
}

impl Namespaces {
    pub fn namespaces(pid: i32) -> ProcResult<Self> {
        let mut ns_path = PathBuf::from(format!("/proc/{}/ns/cgroup", pid));
        let cgroup = since_kernel!(4, 6, 0, readlink(&ns_path))?;

        ns_path.set_file_name("ipc");
        let ipc = since_kernel!(3, 0, 0, readlink(&ns_path))?;

        ns_path.set_file_name("mnt");
        let mnt = since_kernel!(3, 8, 0, readlink(&ns_path))?;

        ns_path.set_file_name("net");
        let net = since_kernel!(3, 0, 0, readlink(&ns_path))?;

        ns_path.set_file_name("pid");
        let pid = since_kernel!(3, 8, 0, readlink(&ns_path))?;

        ns_path.set_file_name("user");
        let user = since_kernel!(4, 6, 0, readlink(&ns_path))?;

        ns_path.set_file_name("uts");
        let uts = since_kernel!(4, 6, 0, readlink(&ns_path))?;

        ns_path.set_file_name("pid_for_children");
        let pid_for_children = since_kernel!(4, 12, 0, readlink(&ns_path))?;

        ns_path.set_file_name("time");
        let time = since_kernel!(5, 6, 0, readlink(&ns_path))?;

        ns_path.set_file_name("time_for_children");
        let time_for_children = since_kernel!(5, 6, 0, readlink(&ns_path))?;
        Ok(Namespaces {
            cgroup,
            ipc,
            mnt,
            net,
            pid,
            user,
            uts,
            pid_for_children,
            time,
            time_for_children,
            _private: (),
        })
    }
}

pub fn readlink(p: &PathBuf) -> ProcResult<Option<String>> {
    // println!("path: {:?}", p.clone());
    match read_link(p) {
        Ok(v) => {
            let inode_string = match v.into_os_string().into_string() {
                Ok(s) => s,
                Err(_) => {
                    return Err(ProcError::Other(format!("Read ns failed: {:?}", p)));
                }
            };
            Ok(Some(inode_string))
        }
        Err(e) => Err(ProcError::from(e)),
    }
}

#[cfg(test)]
mod namespace_tests {
    use super::*;
    use crate::process::Process;

    #[test]
    fn test_task() {
        let me = Process::myself().unwrap();
        let namespace = Namespaces::namespaces(me.pid).unwrap();
        println!("/proc/{}/ns:\n {:?}", me.pid, namespace);
    }
}
