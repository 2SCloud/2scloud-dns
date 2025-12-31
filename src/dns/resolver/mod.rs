use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub(crate) mod stub;

pub(crate) fn check_answer_diff(
    questions: &[QuestionSection],
    answers: &[AnswerSection],
) -> Result<(), SCloudException> {
    for q in questions {
        let found = answers
            .iter()
            .any(|a| a.q_name == q.q_name && a.r_type == q.q_type && a.r_class == q.q_class);

        if !found {
            return Err(SCloudException::SCLOUD_RESOLVER_ANSWER_MISMATCH);
        }
    }

    Ok(())
}

pub(crate) fn check_authority_diff(
    questions: &[QuestionSection],
    authorities: &[AuthoritySection],
) -> Result<(), SCloudException> {
    for record in authorities.iter() {
        for question in questions {
            if !record.q_name.ends_with(question.q_name.as_str()) {
                return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
            }
        }
    }
    Ok(())
}

pub(crate) fn check_additional_diff(
    questions: &[QuestionSection],
    additionals: &[AdditionalSection],
) -> Result<(), SCloudException> {
    for record in additionals.iter() {
        for question in questions {
            if !record.q_name.ends_with(question.q_name.as_str()) {
                return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
            }
        }
    }
    Ok(())
}
