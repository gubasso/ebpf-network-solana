use solana_rbpf::{
    aligned_memory::AlignedMemory, ebpf, elf::Executable, memory_region::{MemoryMapping, MemoryRegion}, program::{BuiltinFunction, BuiltinProgram, FunctionRegistry}, syscalls::SyscallString, verifier::RequisiteVerifier, vm::{Config, EbpfVm, TestContextObject}
};

use std::{fs::File, sync::Arc};
use std::io::Read;

fn main() {

    let mut file = File::open("../ebpf_program/target/sbf-solana-solana/release/ebpf_program.so").expect("Failed to open ELF file");
    let mut elf_bytes = Vec::new();
    file.read_to_end(&mut elf_bytes).expect("Failed to read ELF file");

    let mut function_registry = FunctionRegistry::<BuiltinFunction<TestContextObject>>::default();
    function_registry
        .register_function_hashed(*b"log", SyscallString::vm)
        .unwrap();
    // let loader = std::sync::Arc::new(BuiltinProgram::new_mock());
    let config = Config {
        enable_instruction_meter: true,
        instruction_meter_checkpoint_distance: 1_000_000,
        ..Default::default()
    };
    let loader = Arc::new(BuiltinProgram::new_loader(
        config,
        function_registry,
    ));
    let executable = Executable::<TestContextObject>::from_elf(&elf_bytes, loader.clone()).unwrap();
    executable.verify::<RequisiteVerifier>().unwrap();
    let mut context_object = TestContextObject::new(1);
    let sbpf_version = executable.get_sbpf_version();

    let mut date_str = "2024-04-10".to_string();
    let date_bytes: &mut [u8] = unsafe { date_str.as_bytes_mut() };

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
        MemoryRegion::new_writable(&mut *date_bytes, ebpf::MM_INPUT_START),
    ];

    let memory_mapping = MemoryMapping::new(regions, executable.get_config(), sbpf_version).unwrap();

    let mut vm = EbpfVm::new(loader, sbpf_version, &mut context_object, memory_mapping, stack_len);

    let (instruction_count, result) = vm.execute_program(&executable, true);

    println!("Program executed with {} instructions.", instruction_count);
    println!("Result: {:#?}", result);
}
