//! Text encoding boundaries.
//!
//! Everything this project writes is UTF-8. Two boundaries need explicit care:
//!
//! * **Outbound files** (CSV exports) are UTF-8 **with a BOM**, because Excel on
//!   a Chinese Windows system otherwise decodes them as ANSI and shows mojibake.
//! * **Inbound text** (child-process output, remote error messages) may be GBK
//!   on a Chinese Windows host. Strict UTF-8 is tried first, then GB18030 — a
//!   superset of GBK/GB2312 — so a failed decode never reaches the UI as
//!   `U+FFFD` replacement characters.

use encoding_rs::GB18030;

pub const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Decodes externally produced bytes without ever emitting replacement garbage.
///
/// Order: valid UTF-8 (the common case) → GB18030 (Chinese Windows) → lossy
/// UTF-8 as the last resort for truly undecodable input.
pub fn decode_external_text(bytes: &[u8]) -> String {
    let bytes = strip_utf8_bom(bytes);
    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_owned();
    }
    let (decoded, _, had_errors) = GB18030.decode(bytes);
    if had_errors {
        log::debug!("external text was neither UTF-8 nor GB18030; falling back to lossy UTF-8");
        return String::from_utf8_lossy(bytes).into_owned();
    }
    decoded.into_owned()
}

pub fn has_utf8_bom(bytes: &[u8]) -> bool {
    bytes.starts_with(&UTF8_BOM)
}

/// Removes a leading UTF-8 BOM if present.
pub fn strip_utf8_bom(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(&UTF8_BOM).unwrap_or(bytes)
}

/// Prefixes a UTF-8 BOM exactly once (idempotent for callers that may retry).
pub fn with_utf8_bom(mut bytes: Vec<u8>) -> Vec<u8> {
    if has_utf8_bom(&bytes) {
        return bytes;
    }
    let mut prefixed = Vec::with_capacity(bytes.len() + UTF8_BOM.len());
    prefixed.extend_from_slice(&UTF8_BOM);
    prefixed.append(&mut bytes);
    prefixed
}

#[cfg(test)]
mod tests {
    use super::{decode_external_text, has_utf8_bom, strip_utf8_bom, with_utf8_bom, UTF8_BOM};

    #[test]
    fn decodes_plain_utf8() {
        assert_eq!(decode_external_text("录制已开始".as_bytes()), "录制已开始");
    }

    #[test]
    fn decodes_gbk_console_output() {
        // "中文" encoded as GBK: D6 D0 CE C4
        let gbk = [0xD6_u8, 0xD0, 0xCE, 0xC4];
        assert_eq!(decode_external_text(&gbk), "中文");
    }

    #[test]
    fn decodes_gbk_error_message_with_ascii() {
        // "错误: 无法连接" in GBK, mixed with ASCII punctuation
        let mut bytes = b"error: ".to_vec();
        bytes.extend_from_slice(&[0xB4, 0xED, 0xCE, 0xF3]);
        assert_eq!(decode_external_text(&bytes), "error: 错误");
    }

    #[test]
    fn strips_a_leading_bom_before_decoding() {
        let mut bytes = UTF8_BOM.to_vec();
        bytes.extend_from_slice("开始".as_bytes());
        assert_eq!(decode_external_text(&bytes), "开始");
    }

    #[test]
    fn never_returns_replacement_characters_for_valid_gbk() {
        let gbk = [0xC2, 0xBC, 0xD6, 0xC6]; // "录制"
        let decoded = decode_external_text(&gbk);
        assert!(!decoded.contains('\u{FFFD}'), "decoded: {decoded}");
        assert_eq!(decoded, "录制");
    }

    #[test]
    fn adds_the_bom_exactly_once() {
        let once = with_utf8_bom(b"a,b\n1,2\n".to_vec());
        assert!(has_utf8_bom(&once));
        assert_eq!(&once[..3], &UTF8_BOM);

        let twice = with_utf8_bom(once.clone());
        assert_eq!(twice, once);
        assert_eq!(strip_utf8_bom(&twice), b"a,b\n1,2\n");
    }

    #[test]
    fn keeps_utf8_payload_byte_identical_apart_from_the_bom() {
        let payload = "时间,在线人数\n2026-11-21,42\n".as_bytes().to_vec();
        let encoded = with_utf8_bom(payload.clone());
        assert_eq!(&encoded[3..], &payload[..]);
        assert_eq!(std::str::from_utf8(&encoded[3..]).unwrap(), "时间,在线人数\n2026-11-21,42\n");
    }
}
