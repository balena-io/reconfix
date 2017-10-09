

use std::fs::{self, File, DirEntry};
use std::error;
use std::result;
use std::str::FromStr;
use std::os::linux::fs::MetadataExt;
use std::io::Read;

use io::{Plugin, Content};
use common::{FileNode, Partition};
use error::*;

/// The default Reconfix plugin
pub struct HostFile {}

impl Plugin for HostFile {
    fn open(
        &self,
        node: &FileNode,
    ) -> result::Result<Box<Content>, Box<error::Error + Send + Sync>> {
        let path = node.path.join("/");
        let file = File::open(path)?;

        Ok(Box::new(file))
    }
}

impl Content for File {}

struct Device {
    pub name: String,
    pub major: u32,
    pub minor: u32,
    pub partition: Option<Partition>,
    pub children: Vec<Device>,
}

fn get_root_device(devices: &[Device], parent: Option<&Device>) -> Result<()> {
    let metadata = fs::metadata("/")
        .chain_err(|| "unable to stat root directory")?;

    let device = metadata.st_dev();

    Ok(())
}

fn get_devices() -> Result<Vec<Device>> {
    let dirs = fs::read_dir("/sys/block")
        .chain_err(|| "unable to read block devices")?;

    let mut parents = Vec::new();
    
    for dir in dirs {
        let next_dir = dir.chain_err(|| "unable to read next entry")?;
        let mut device = read_device(&next_dir)
            .chain_err(|| "unable to read device")?;
        
        let subdirs = fs::read_dir(next_dir.path())
            .chain_err(|| "unable to read sub-devices")?;

        for dir in subdirs {
            let next_dir = dir.chain_err(|| "unable to read next entry")?;
            let subname = next_dir.file_name().to_str()
                .chain_err(|| "invalid sub-device name")?
                .to_string();

            if subname.starts_with(&device.name) {
                let sub_device = read_device(&next_dir)?;
                device.children.push(sub_device);
            }
        }

        parents.push(device);
    }

    Ok(parents)
}

fn read_device(dir: &DirEntry) -> Result<Device> {
    let canonical = dir.path().canonicalize()
        .chain_err(|| "unable to locate device")?;

    let name = canonical.file_name()
        .and_then(|os| os.to_str())
        .ok_or_else(|| "invalid device dir")?
        .to_string();
    
    let device = canonical.join("dev");
    let mut dev_file = File::open(&device)
        .chain_err(|| "unable to open device file")?;

    let mut device_str = String::new();
    dev_file.read_to_string(&mut device_str)
        .chain_err(|| "unable to read device data")?;

    let pair = device_str.splitn(2, ':').collect::<Vec<_>>();
    let major = pair.get(0).ok_or_else(|| "missing major number".into())
        .and_then(|m| u32::from_str(m).chain_err(|| "invalid major number"))?;
    let minor = pair.get(1).ok_or_else(|| "missing minor number".into())
        .and_then(|m| u32::from_str(m).chain_err(|| "invalid minor number"))?;
    
    let partition = canonical.join("partition");
    let parition_num = match partition.is_file() {
        false => None,
        true => {
            let mut part_file = File::open(&partition)
            .chain_err(|| "unable to open partition file")?;
        
            let mut partition_str = String::new();
            part_file.read_to_string(&mut partition_str)
                .chain_err(|| "unable to read partition data")?;

            let p = u8::from_str(&partition_str)
                .chain_err(|| "unable to parse partition")?;
            
            Some(Partition::new(p))
        }
    };

    Ok(Device {
        name: name,
        major: major,
        minor: minor,
        partition: parition_num,
        children: vec![],
    })
}
