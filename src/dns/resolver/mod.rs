pub(crate) mod stub;

use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub(crate) fn check_answer_diff(
    questions: &[QuestionSection],
    answers: &[AnswerSection],
    authorities: &[AuthoritySection],
    additionals: &[AdditionalSection],
) -> Result<(), SCloudException> {
    for q in questions {
        let found_in_answers = answers.iter().any(|a| a.q_name == q.q_name && a.r_class == q.q_class);
        let found_in_authorities = authorities.iter().any(|a| a.q_name == q.q_name && a.q_class == q.q_class);
        let found_in_additionals = additionals.iter().any(|a| a.q_name == q.q_name && a.q_class == q.q_class);

        if !found_in_answers && !found_in_authorities && !found_in_additionals {
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
        if !questions.iter().any(|q| record.q_name == q.q_name) {
            return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
        }
    }
    Ok(())
}

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
