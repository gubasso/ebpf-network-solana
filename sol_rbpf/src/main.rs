use solana_rbpf::{
    aligned_memory::AlignedMemory,
    ebpf,
    elf::Executable,
    memory_region::{MemoryMapping, MemoryRegion},
    program::{BuiltinProgram, FunctionRegistry, SBPFVersion},
    verifier::RequisiteVerifier,
    vm::{Config, EbpfVm, TestContextObject},
};

use std::fs::File;
use std::io::Read;

fn main() {

    let mut file = File::open("ebpf_program.so").expect("Failed to open ELF file");
    let mut elf_bytes = Vec::new();
    file.read_to_end(&mut elf_bytes).expect("Failed to read ELF file");


    // let mem = &mut [
    //     0xaa, 0xbb, 0x11, 0x22, 0xcc, 0xdd
    // ];
    let mem = &mut [0u8; 1024];

    let loader = std::sync::Arc::new(BuiltinProgram::new_mock());
    // let function_registry = FunctionRegistry::default();
    let mut executable = Executable::<TestContextObject>::from_elf(&mut elf_bytes, loader.clone()).unwrap();
    executable.verify::<RequisiteVerifier>().unwrap();
    let mut context_object = TestContextObject::new(1);
    let sbpf_version = executable.get_sbpf_version();

    let mut stack = AlignedMemory::<{ebpf::HOST_ALIGN}>::zero_filled(executable.get_config().stack_size());
    let stack_len = stack.len();
    let mut heap = AlignedMemory::<{ebpf::HOST_ALIGN}>::with_capacity(0);

    let regions: Vec<MemoryRegion> = vec![
        executable.get_ro_region(),
        MemoryRegion::new_writable(
        stack.as_slice_mut(),
        ebpf::MM_STACK_START,
        ),
        MemoryRegion::new_writable(heap.as_slice_mut(), ebpf::MM_HEAP_START),
        MemoryRegion::new_writable(mem, ebpf::MM_INPUT_START),
    ];

    let memory_mapping = MemoryMapping::new(regions, executable.get_config(), sbpf_version).unwrap();

    let mut vm = EbpfVm::new(loader, sbpf_version, &mut context_object, memory_mapping, stack_len);

    let (instruction_count, result) = vm.execute_program(&executable, true);

    println!("Program executed with {} instructions.", instruction_count);
}
