use crate::storage_format::FileContent;

use std::sync::{
    Mutex,
    MutexGuard,
};

pub enum ECapability {
    Boolean(bool),
    Number(i16),
    String(String),
}

pub struct Capability {
    pub(crate) capability: ECapability,
    pub(crate) supported: bool,
}

impl Capability {
    pub fn capability(self) -> Option<ECapability> {
        match self.supported {
            true => Some(self.capability),
            false => None
        }
    }
}

pub struct CapabilityBuilder {
    content: Option<FileContent>,
}

static mut CAPABILITY_BUILDER: Mutex<CapabilityBuilder> = Mutex::new(CapabilityBuilder { content: None, });

impl CapabilityBuilder {
    pub fn initial_instance(content: FileContent) {
        unsafe {
            CAPABILITY_BUILDER.lock().as_mut().unwrap().content = Some(content)
        }
    }

    pub fn instance() -> MutexGuard<'static, CapabilityBuilder> {
        unsafe {
            CAPABILITY_BUILDER.lock().unwrap()
        }
    }

    pub fn boolean(&self, position: usize) -> Capability {
        let capability = self.content.as_ref().unwrap().boolean_flags.get(position).unwrap();
        let capability = if capability > &0 { true } else { false };
        let capability = ECapability::Boolean(capability);
        let supported = true;

        Capability {
            capability,
            supported,
        }
    }

    pub fn number(&self, position: usize) -> Capability {
        let capability = self.content.as_ref().unwrap().numbers.get(position).unwrap();
        let supported = match capability {
            -1 | -2 => false,
            _ if capability > &0 => true,
            _ => false,
        };
        let capability = ECapability::Number(*capability);
        Capability {
            capability,
            supported,
        }
    }

    pub fn string(&mut self, position: usize) -> Capability {
        let offset = self.content.as_ref().unwrap().strings_offset.get(position).unwrap();
        //let string = self.content.string_table.drain(*offset as usize..).collect();
        let string = self.content.as_ref().unwrap().string_table[*offset as usize..].to_vec();
        let capability = ECapability::String(String::from_utf8(string).unwrap());
        let supported = true;

        Capability {
            capability,
            supported,
        }
    }
}

mod capabilities;

pub use capabilities::*;