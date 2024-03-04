pub mod mailbox {

use std::io::BufRead;
use std::io::BufReader;

#[repr(C)]
pub struct PcieBdf {
    bus: u8,
    device: u8,
    function: u8,
}

#[repr(C)]
pub struct PCIE_CLASS_CODE {
    prog_if: u8,
    sub_class_code: u8,
    base_class_code: u8,
}

#[repr(C, packed)]
pub struct BAR {
    region_type: u32,
    locatable: u32,
    prefetchable: u32,
    base_address: u32,
}

#[repr(C)]
pub struct PCIE_CONFIG_HDR {
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: i16,
    rev_id: u8,
    class_code: PCIE_CLASS_CODE,
    misc: u32,
    base_address_registers: [BAR; 6],
    misc2: [i32; 6],
}

#[repr(C)]
pub struct DVSEC_HDR1 {
    dvsec_vendor_id: u16,
    dvsec_rev: u16,
    dvsec_length: u16,
}

#[repr(C)]
pub struct DVSEC_HDR2 {
    dvsec_id: u16,
}

#[repr(C)]
pub struct PCIE_EXT_CAP_HDR {
    pcie_ext_cap_id: u16,
    cap_ver: u16,
    next_cap_ofs: u16,
    dvsec_hdr1: DVSEC_HDR1,
    dvsec_hdr2: DVSEC_HDR2,
}

#[repr(C)]
pub struct REGISTER_OFFSET_LOW {
    register_bir: u8,
    rsvdp: u8,
    register_block_identifier: u8,
    register_block_offset_low: u16,
}

#[repr(C)]
pub struct REGISTER_OFFSET_HIGH {
    register_block_offset_high: u32,
}

#[repr(C)]
pub struct REGISTER_BLOCK {
    register_offset_low: REGISTER_OFFSET_LOW,
    register_offset_high: REGISTER_OFFSET_HIGH,
}

#[repr(C)]
pub struct registerLocator {
    pcie_ext_cap_hdr: PCIE_EXT_CAP_HDR,
    rsvdp: u16,
    register_block: [REGISTER_BLOCK; 4],
}

#[repr(C)]
pub struct DEVICE_CAPABILITIES_ARRAY_REGISTER {
    capability_id: u16,
    version: u8,
    reserved: u8,
    capabilities_count: u16,
    reserved2: [u8; 10],
}

#[repr(C)]
pub struct DEVICE_CAPABILITIES_HEADER {
    capability_id: u16,
    version: u8,
    reserved: u8,
    offset: u32,
    length: u32,
    reserved2: u32,
}

#[repr(C)]
pub struct MemoryDeviceRegisters {
    device_capabilities_array_register: DEVICE_CAPABILITIES_ARRAY_REGISTER,
    device_capability_header: [DEVICE_CAPABILITIES_HEADER; 3],
    device_capability: [u8; 4096 - 16 - 16 * 3],
}

#[repr(C)]
pub struct mailbox_capabilities_register {
    payload_size: u32,
    mb_doorbell_int_capable: u32,
    background_command_complete_interupt_capable: u32,
    intr_msg_number: u32,
    mb_ready_time: u32,
    r#type: u32,
    reserved: u32,
}

#[repr(C)]
pub struct mailbox_control_register {
    doorbell: u32,
    doorbell_interrupt_mode: u32,
    background_command_complete_interupt: u32,
    reserved: u32,
}

#[repr(C)]
pub struct mailbox_command_register {
    opcode: u64,
    payload_size: u64,
    reserved: u64,
}

#[repr(C)]
pub struct mailbox_status_register {
    background_operation_status: u64,
    reserved: u64,
    return_code: u64,
    vendor_specific_ext_status: u64,
}

#[repr(C)]
pub struct mailbox_background_command_status_register {
    last_opcode: u64,
    percent_complete: u64,
    reserved: u64,
    return_code: u64,
    vendor_specific_ext_status: u64,
}

#[repr(C)]
pub struct mailbox_registers {
    mb_capabilities: mailbox_capabilities_register,
    mb_control: mailbox_control_register,
    command_register: mailbox_command_register,
    mb_status: mailbox_status_register,
    background_command_status_register: mailbox_background_command_status_register,
    command_payload_registers: [u32; 512],
}

pub struct U64Field {
    offset: i32,
    bitwidth: i32,
}

pub fn mask_u64field(u: &U64Field) -> u64 {
    ((1u64 << u.bitwidth) - 1) << u.offset
}

pub fn read_u64field(u: &U64Field, reg: u64) -> u64 {
    (reg >> u.offset) & ((1u64 << u.bitwidth) - 1)
}

pub fn write_u64field(u: &U64Field, reg: &mut u64, val: u64) {
    *reg = (*reg & !mask_u64field(u)) | ((val << u.offset) & mask_u64field(u));
}

pub struct U32Field {
    offset: i32,
    bitwidth: i32,
}

pub fn mask_u32field(u: &U32Field) -> u32 {
    ((1u32 << u.bitwidth) - 1) << u.offset
}

pub fn read_u32field(u: &U32Field, reg: u32) -> u32 {
    (reg >> u.offset) & ((1u32 << u.bitwidth) - 1)
}

pub fn write_u32field(u: &U32Field, reg: &mut u32, val: u32) {
    *reg = (*reg & !mask_u32field(u)) | ((val << u.offset) & mask_u32field(u));
}   

#[cfg(DEBUG)]
const DEBUG: bool = true;

const CXL_DVSEC_CAPABILITY_ID: u16 = 0x0019;
const CXL_DVSEC_VERSION: u8 = 0x2;
const CXL_DEVICE_REGISTERS_ID: u8 = 0x03;
const CXL_VENDOR_ID: u16 = 0x1E98;

static mut PCI_MMCONFIG_BASE_ADDR: u64 = 0x0;

fn extract_bdf(pcie_bdf: &str) -> PcieBdf {
    let mut bdf = PcieBdf {
        bus: 0,
        device: 0,
        function: 0,
    };
    let parts: Vec<&str> = pcie_bdf.split(':').collect();
    if parts.len() == 3 {
        bdf.bus = u8::from_str_radix(parts[0], 16).unwrap_or(0);
        bdf.device = u8::from_str_radix(parts[1], 16).unwrap_or(0);
        bdf.function = u8::from_str_radix(parts[2], 16).unwrap_or(0);
    }
    bdf
}

fn print_buffer_hex(buf: &[u8]) {
    for (i, &byte) in buf.iter().enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
    println!();
}

pub fn get_pci_mm_config(pcie_bdf: &str) {
    let mut file = match std::fs::File::open("/proc/iomem") {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut pci_mmconfig_base_addr: Option<u64> = None;
    let bdf = extract_bdf(pcie_bdf);

    while let Ok(bytes_read) = reader.read_line(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        if buffer.contains("ECAM") {
            if let Some(addr) = buffer.split('-').next() {
                if let Ok(parsed_addr) = u64::from_str_radix(addr.trim(), 16) {
                    pci_mmconfig_base_addr = Some(parsed_addr);
                    break;
                }
            }
        }
        buffer.clear();
    }

    if let Some(base_addr) = pci_mmconfig_base_addr {
        unsafe {
            PCI_MMCONFIG_BASE_ADDR = base_addr;
        }
    }

    let address = unsafe {
        PCI_MMCONFIG_BASE_ADDR + ((bdf.bus as u64) << 20) + ((bdf.device as u64) << 15) + ((bdf.function as u64) << 12)
    };
    println!("{}: PCIe BDF: {} based address = {:X}", "get_pci_mm_config", pcie_bdf, address);
}

fn print_registers(map_base: *const u32, size: usize) {
    let virt_addr = unsafe { std::slice::from_raw_parts(map_base, size / 4) };
    for &value in virt_addr {
        println!("0x{:08X}", value);
    }
    println!();
}


/*
fn parse_struct<T>(src: &[u8]) -> T {
    let mut dst = Vec::with_capacity(std::mem::size_of::<T>());
    dst.extend_from_slice(&src[..std::mem::size_of::<T>()]);
    unsafe { std::mem::transmute_copy(&dst[..]) }
}
*/
}
// Usage example
// fn main() {
//     use crate::get_pci_mm_config;

//     let pcie_bdf = "00:01:02";
//     get_pci_mm_config(pcie_bdf);
// }



