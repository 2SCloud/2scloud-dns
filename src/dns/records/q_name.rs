use crate::exceptions::SCloudException;

// TODO: check this function because QNAME start at byte 12, and he seems to not take that in count.
/// Proper DNS QNAME parsing (length-label encoded + compression support)
pub(crate) fn parse_qname(buf: &[u8]) -> Result<(String, usize), SCloudException> {
    let mut labels = Vec::new();
    let mut pos = 0;
    let mut jumped = false;
    let mut consumed = 0;

    loop {
        // TODO: check that seems weird cauz the buffer is normally always greater than 0.
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
                consumed = pos + 2;
            }

            return Ok((parse_qname_at(buf, offset)?, consumed));
        }

        // End of QNAME
        if len == 0 {
            if !jumped {
                consumed = pos + 1;
            }
            break;
        }

        pos += 1;
        if pos + len as usize > buf.len() {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME);
        }

        let label = &buf[pos..pos + len as usize];
        let s = String::from_utf8(label.to_vec())
            .map_err(|_| SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED)?;

        labels.push(s);

        pos += len as usize;
    }

    Ok((labels.join("."), consumed))
}

/// Helper: parse a qname at an offset (used for compression)
pub(crate) fn parse_qname_at(buf: &[u8], offset: usize) -> Result<String, SCloudException> {
    let (name, _consumed) = parse_qname(&buf[offset..])?;
    Ok(name)
}
