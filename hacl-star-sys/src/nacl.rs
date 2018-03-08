/* automatically generated by rust-bindgen */

pub type __uint8_t = ::libc::c_uchar;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __uint64_t = ::libc::c_ulong;
pub type Prims_int = i32;
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_box_NONCEBYTES"]
    pub static mut NaCl_crypto_box_NONCEBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_box_PUBLICKEYBYTES"]
    pub static mut NaCl_crypto_box_PUBLICKEYBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_box_SECRETKEYBYTES"]
    pub static mut NaCl_crypto_box_SECRETKEYBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_box_MACBYTES"]
    pub static mut NaCl_crypto_box_MACBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_secretbox_NONCEBYTES"]
    pub static mut NaCl_crypto_secretbox_NONCEBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_secretbox_KEYBYTES"]
    pub static mut NaCl_crypto_secretbox_KEYBYTES: Prims_int;
}
extern "C" {
    #[link_name = "\u{1}NaCl_crypto_secretbox_MACBYTES"]
    pub static mut NaCl_crypto_secretbox_MACBYTES: Prims_int;
}
extern "C" {
    pub fn NaCl_crypto_secretbox_detached(
        c: *mut u8,
        mac: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_secretbox_open_detached(
        m: *mut u8,
        c: *mut u8,
        mac: *mut u8,
        clen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_secretbox_easy(
        c: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_secretbox_open_easy(
        m: *mut u8,
        c: *mut u8,
        clen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_beforenm(k1: *mut u8, pk: *mut u8, sk: *mut u8) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_detached_afternm(
        c: *mut u8,
        mac: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_detached(
        c: *mut u8,
        mac: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        pk: *mut u8,
        sk: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_open_detached(
        m: *mut u8,
        c: *mut u8,
        mac: *mut u8,
        mlen: u64,
        n1: *mut u8,
        pk: *mut u8,
        sk: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_easy_afternm(
        c: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_easy(
        c: *mut u8,
        m: *mut u8,
        mlen: u64,
        n1: *mut u8,
        pk: *mut u8,
        sk: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_open_easy(
        m: *mut u8,
        c: *mut u8,
        mlen: u64,
        n1: *mut u8,
        pk: *mut u8,
        sk: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_open_detached_afternm(
        m: *mut u8,
        c: *mut u8,
        mac: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
extern "C" {
    pub fn NaCl_crypto_box_open_easy_afternm(
        m: *mut u8,
        c: *mut u8,
        mlen: u64,
        n1: *mut u8,
        k1: *mut u8,
    ) -> u32;
}
