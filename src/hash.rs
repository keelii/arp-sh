use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use crc::{Crc, CRC_16_ARC, CRC_32_ISO_HDLC};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::Sha256;
use sha3::{Sha3_384, Sha3_512};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct HashFormData {
    pub content: String,
    // pub file: Multipart,
}


#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct I16Hex {
    int: u16,
    hex: String,
}
#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct I32Hex {
    int: u32,
    hex: String,
}
#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct HexB64 {
    hex: String,
    b64: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct HashResult {
    text: String,
    md5: String,
    sha1: HexB64,
    sha256: HexB64,
    sha512: HexB64,
    sha384: HexB64,
    crc16: I16Hex,
    crc32: I32Hex,
}

pub fn hash_bytes(bytes: &[u8]) -> HashResult {
    let mut md5_hasher = Md5::new();
    md5_hasher.update(bytes);
    let md5_hash = md5_hasher.finalize();
    let md5_ret = format!("{:x}", md5_hash);

    let sha1_digest = Sha1::digest(bytes);

    let sha1_hex = format!("{:x}", sha1_digest);
    let sha1_b64 = BASE64_STANDARD.encode(sha1_digest);

    let sha256_digest = Sha256::digest(bytes);
    let sha256_hex = format!("{:x}", sha256_digest);
    let sha256_b64 = BASE64_STANDARD.encode(sha256_digest);

    let sha512_digest = Sha3_512::digest(bytes);
    let sha512_hex = format!("{:x}", sha512_digest);
    let sha512_b64 = BASE64_STANDARD.encode(sha512_digest);

    let sha384_digest = Sha3_384::digest(bytes);
    let sha384_hex = format!("{:x}", sha384_digest);
    let sha384_b64 = BASE64_STANDARD.encode(sha384_digest);

    let crc16: Crc<u16> = Crc::<u16>::new(&CRC_16_ARC);
    let crc16_int = crc16.checksum(bytes);
    let crc16_hex = format!("{:#06x}", crc16_int);

    let crc32: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let crc32_int = crc32.checksum(bytes);
    let crc32_hex = format!("{:#010x}", crc32_int);

    HashResult {
        text: String::from_utf8_lossy(bytes).to_string(),
        md5: md5_ret,
        sha1: HexB64 {
            hex: sha1_hex,
            b64: sha1_b64,
        },
        sha256: HexB64 {
            hex: sha256_hex,
            b64: sha256_b64,
        },
        sha512: HexB64 {
            hex: sha512_hex,
            b64: sha512_b64,
        },
        sha384: HexB64 {
            hex: sha384_hex,
            b64: sha384_b64,
        },
        crc16: I16Hex {
            int: crc16_int,
            hex: crc16_hex,
        },
        crc32: I32Hex {
            int: crc32_int,
            hex: crc32_hex,
        },
    }
}
