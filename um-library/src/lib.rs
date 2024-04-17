struct UniversalMachine{
    registers: [u32; 8],
    memory_segments: Vec<Vec<u32>>,
    io_device: IoDevice,
    program_counter: usize,
}

struct IoDevice{

}

