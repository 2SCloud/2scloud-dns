pub(crate) mod stub;

use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

/// Check that each question has at least one corresponding record
/// in the answer, authority, or additional sections.
///
/// This prevents responses that do not actually answer the query.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_answer_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::answer::AnswerSection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
/// }];
///
/// let answers = vec![AnswerSection {
///     q_name: "example.com".to_string(),
///     r_type: DNSRecordType::A,
///     r_class: DNSClass::IN,
///     ttl: 300,
///     rdlength: 4,
///     rdata: vec![93, 184, 216, 34],
/// }];
///
/// assert!(check_answer_diff(&questions, &answers, &[], &[]).is_ok());
/// ```
pub(crate) fn check_answer_diff(
    questions: &[QuestionSection],
    answers: &[AnswerSection],
    authorities: &[AuthoritySection],
    additionals: &[AdditionalSection],
) -> Result<(), SCloudException> {
    for q in questions {
        let found_in_answers = answers
            .iter()
            .any(|a| a.q_name == q.q_name && a.r_class == q.q_class);
        let found_in_authorities = authorities
            .iter()
            .any(|a| a.q_name == q.q_name && a.q_class == q.q_class);
        let found_in_additionals = additionals
            .iter()
            .any(|a| a.q_name == q.q_name && a.q_class == q.q_class);

        if !found_in_answers && !found_in_authorities && !found_in_additionals {
            return Err(SCloudException::SCLOUD_RESOLVER_ANSWER_MISMATCH);
        }
    }

    Ok(())
}

/// Ensure that authority records belong to the same zone
/// as the original DNS questions.
///
/// This prevents out-of-zone NS records.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_authority_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::authority::AuthoritySection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::NS,
///     q_class: DNSClass::IN,
/// }];
///
/// let authorities = vec![AuthoritySection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::NS,
///     q_class: DNSClass::IN,
///     ttl: 3600,
///     ns_name: "ns1.example.com".to_string(),
/// }];
///
/// assert!(check_authority_diff(&questions, &authorities).is_ok());
/// ```
pub(crate) fn check_authority_diff(
    questions: &[QuestionSection],
    authorities: &[AuthoritySection],
) -> Result<(), SCloudException> {
    for record in authorities.iter() {
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
        }
    }
    Ok(())
}

/// Ensure that additional records correspond to the original questions.
///
/// This is commonly used to validate glue records.
///
/// # Exemple :
/// ```
/// use crate::dns::resolver::check_additional_diff;
/// use crate::dns::packet::question::QuestionSection;
/// use crate::dns::packet::additional::AdditionalSection;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let questions = vec![QuestionSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
/// }];
///
/// let additionals = vec![AdditionalSection {
///     q_name: "example.com".to_string(),
///     q_type: DNSRecordType::A,
///     q_class: DNSClass::IN,
///     ttl: 300,
///     rdlength: 4,
///     rdata: vec![192, 0, 2, 1],
/// }];
///
/// assert!(check_additional_diff(&questions, &additionals).is_ok());
/// ```
pub(crate) fn check_additional_diff(
    questions: &[QuestionSection],
    additionals: &[AdditionalSection],
) -> Result<(), SCloudException> {
    for record in additionals.iter() {
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
        }
    }
    Ok(())
}
