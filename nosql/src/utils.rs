pub fn u32_to_bytes(num: u32) -> [u8; 4] {
    return [
        ((num >> 24) & 255) as u8,
        ((num >> 16) & 255) as u8,
        ((num >> 8 ) & 255) as u8,
        ( num        & 255) as u8
    ]
}

pub fn bytes_to_usize(bvec: Vec<u8>) -> usize {
    return ((bvec[0] as usize) << 24)
        + ((bvec[1] as usize) << 16)
        + ((bvec[2] as usize) << 8)
        + bvec[3] as usize;
}