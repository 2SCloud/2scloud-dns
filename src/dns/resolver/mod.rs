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
    for question in questions {
        let mut found = false;

        for answer in answers {
            let mut matched = false;

            for question in questions {
                if answer.q_name == question.q_name
                    && answer.r_type == question.q_type
                    && answer.r_class == question.q_class
                {
                    matched = true;
                    break;
                }
            }

            if !matched {
                return Err(SCloudException::SCLOUD_RESOLVER_ANSWER_MISMATCH);
            }
        }
    }

    Ok(())
}

pub(crate) fn check_authority_diff(
    zone: &str,
    authorities: &[AuthoritySection]
) -> Result<(), SCloudException> {
    for record in authorities.iter() {
        if !record.q_name.ends_with(zone) {
            return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
        }
    }
    Ok(())
}

pub(crate) fn check_additional_diff(
    zone: &str,
    additionals: &[AdditionalSection]
) -> Result<(), SCloudException> {
    for record in additionals.iter() {
        if !record.q_name.ends_with(zone) {
            return Err(SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE);
        }
    }
    Ok(())
}
