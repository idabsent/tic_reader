use tic_reader::storage_format::FileContent;
use tic_reader::terminal_capabilities::*;

fn main() {
     let content = FileContent::new("/usr/share/terminfo/a/alacritty").unwrap();

     CapabilityBuilder::initial_instance(content);
     let left_marg = auto_left_margin().capability().unwrap();
     let right_marg = auto_right_margin().capability().unwrap();

     if let ECapability::Boolean(cap) = left_marg {
         println!("auto_left_margin capability: {cap}")
     };

     if let ECapability::Boolean(cap) = right_marg {
         println!("auto_right_margin capability: {cap}")
     };
}