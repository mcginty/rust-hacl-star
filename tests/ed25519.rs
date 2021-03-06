extern crate hacl_star;

use hacl_star::ed25519;


const SK11: [u8; 32] = [
    0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd,
    0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4,
    0x92, 0xec, 0x2c, 0xc4, 0x44, 0x49,
    0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19,
    0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
    0x7f, 0x60
];
const PK11: [u8; 32] = [
    0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1,
    0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3,
    0xc9, 0x64, 0x07, 0x3a, 0x0e, 0xe1,
    0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25,
    0xaf, 0x02, 0x1a, 0x68, 0xf7, 0x07,
    0x51, 0x1a
];
const MSG11: [u8; 0] = [];
const SIG11: [u8; 64] = [
    0xe5, 0x56, 0x43, 0x00, 0xc3, 0x60,
    0xac, 0x72, 0x90, 0x86, 0xe2, 0xcc,
    0x80, 0x6e, 0x82, 0x8a, 0x84, 0x87,
    0x7f, 0x1e, 0xb8, 0xe5, 0xd9, 0x74,
    0xd8, 0x73, 0xe0, 0x65, 0x22, 0x49,
    0x01, 0x55, 0x5f, 0xb8, 0x82, 0x15,
    0x90, 0xa3, 0x3b, 0xac, 0xc6, 0x1e,
    0x39, 0x70, 0x1c, 0xf9, 0xb4, 0x6b,
    0xd2, 0x5b, 0xf5, 0xf0, 0x59, 0x5b,
    0xbe, 0x24, 0x65, 0x51, 0x41, 0x43,
    0x8e, 0x7a, 0x10, 0x0b
];

#[test]
fn test_ed25519() {
    let pk = ed25519::SecretKey(SK11).read_public();
    assert_eq!(&pk.0, &PK11);

    let mut sig = ed25519::SecretKey(SK11).signature(&MSG11);
    assert_eq!(&sig.0[..], &SIG11[..]);

    assert!(ed25519::PublicKey(PK11).verify(&MSG11, &sig));

    sig.0[23] ^= 0x01;
    assert!(!ed25519::PublicKey(PK11).verify(&MSG11, &sig));
}
