pub unsafe fn u32_to_vec_u8(x: u32, size: usize) -> Vec<u8> {
    let ptr_u32 = &x as *const u32;
    let ptr_u8 = ptr_u32 as *const u8;
    let mut bytes:Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        bytes.push(*ptr_u8.offset(i as isize));
    }
    bytes
}

pub unsafe fn vec_u8_to_u32(bytes: Vec<u8>) -> u32 {
    *(bytes.as_ptr() as *const u32)
}