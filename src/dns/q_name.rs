use crate::exceptions::SCloudException;

/// Parse a DNS QNAME from a DNS message buffer.
///
/// This function supports:
/// - Standard DNS label encoding
/// - Name compression using pointers (RFC 1035, section 4.1.4)
///
/// # Arguments
/// * `buf` - Full DNS message buffer
/// * `pos` - Offset in the buffer where the QNAME starts
///
/// # Returns
/// * `(String, usize)`
///   - Parsed domain name (e.g. "www.example.com")
///   - Number of bytes consumed at the original position
///
/// # Errors
/// Returns an error if:
/// - The buffer is too short
/// - A label length is invalid
/// - Compression pointers are malformed
///
/// # Notes
/// - When compression is used, the returned `usize` corresponds
///   to the position right after the pointer (not the expanded name).
/// - The caller is responsible for passing the correct initial offset
///   (e.g. 12 for the first QNAME in a DNS packet).
pub(crate) fn parse_qname(buf: &[u8], mut pos: usize) -> Result<(String, usize), SCloudException> {
    let mut labels = Vec::new();
    let mut jumped = false;
    let mut end_pos = pos;

    loop {
        if pos >= buf.len() {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF);
        }

        let len = buf[pos];

        // Compression 0xC0xx
        if len & 0xC0 == 0xC0 {
            if pos + 1 >= buf.len() {
                return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME);
            }

            let offset = (((len as u16 & 0x3F) << 8) | buf[pos + 1] as u16) as usize;

            if !jumped {
                end_pos = pos + 2;
            }

            jumped = true;

            let (name, _) = parse_qname(buf, offset)?;
            labels.extend(name.split('.').map(|s| s.to_string()));
            break;
        }

        if len == 0 {
            if !jumped {
                end_pos = pos + 1;
            }
            break;
        }

        pos += 1;

        if pos + len as usize > buf.len() {
            return Err(
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF,
            );
        }

        let label = &buf[pos..pos + len as usize];
        let s = String::from_utf8(label.to_vec())
            .map_err(|_| SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME)?;

        labels.push(s);
        pos += len as usize;
    }

    Ok((labels.join("."), end_pos))
}

/// Parse a DNS QNAME at a specific offset and return only the name.
///
/// This is mainly used internally when resolving compression pointers.
pub(crate) fn parse_qname_at(buf: &[u8], offset: usize) -> Result<String, SCloudException> {
    let (name, _consumed) = parse_qname(&buf, offset)?;
    Ok(name)
}
