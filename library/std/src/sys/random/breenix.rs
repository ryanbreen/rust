//! Random data from Breenix's `getrandom`.
//!
//! Without flags, `getrandom` waits until the kernel's pool has been credited
//! with enough entropy, so `fill_bytes` (behind `DefaultRandomSource`) never
//! returns predictable data. `HashMap` keys only resist collision attacks; a
//! program that builds a map early in boot must not wait for the pool, so they
//! use `GRND_INSECURE`, which returns at once.

use crate::sys::pal::os::errno;

fn getrandom(mut bytes: &mut [u8], flags: libc::c_uint) {
    while !bytes.is_empty() {
        let ret = unsafe { libc::getrandom(bytes.as_mut_ptr().cast(), bytes.len(), flags) };
        if ret < 0 {
            let err = errno();
            if err == libc::EINTR {
                continue;
            }
            panic!("failed to generate random data: getrandom returned errno {err}");
        }
        bytes = &mut bytes[ret as usize..];
    }
}

pub fn fill_bytes(bytes: &mut [u8]) {
    getrandom(bytes, 0);
}

pub fn hashmap_random_keys() -> (u64, u64) {
    let mut bytes = [0; 16];
    getrandom(&mut bytes, libc::GRND_INSECURE);
    let k1 = u64::from_ne_bytes(bytes[..8].try_into().unwrap());
    let k2 = u64::from_ne_bytes(bytes[8..].try_into().unwrap());
    (k1, k2)
}
