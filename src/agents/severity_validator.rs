use crate::domain::SeverityAssessment;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum SeverityValidationError {
    #[error("confidence must be between 0.0 and 1.0, got {value}")]
    InvalidConfidence { value: f32 },

    #[error("severity justification must not be empty")]
    EmptyJustification,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SeverityValidator;

impl SeverityValidator {
    pub fn validate(&self, assessment: &SeverityAssessment) -> Result<(), SeverityValidationError> {
        validate_confidence(assessment.confidence)?;

        if assessment.justification.trim().is_empty() {
            return Err(SeverityValidationError::EmptyJustification);
        }

        Ok(())
    }
}

fn validate_confidence(confidence: f32) -> Result<(), SeverityValidationError> {
    if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
        return Err(SeverityValidationError::InvalidConfidence { value: confidence });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::domain::{Severity, SeverityAssessment};

    use super::*;

    fn assessment(confidence: f32) -> SeverityAssessment {
        SeverityAssessment {
            severity: Severity::Medium,

            confidence,

            justification: "Multiple related events".into(),
        }
    }

    #[test]
    fn accepts_zero_confidence() {
        let validator = SeverityValidator;

        validator
            .validate(&assessment(0.0))
            .expect("0.0 should be valid");
    }

    #[test]
    fn accepts_one_confidence() {
        let validator = SeverityValidator;

        validator
            .validate(&assessment(1.0))
            .expect("1.0 should be valid");
    }

    #[test]
    fn accepts_confidence_between_bounds() {
        let validator = SeverityValidator;

        validator
            .validate(&assessment(0.73))
            .expect("0.73 should be valid");
    }

    #[test]
    fn rejects_confidence_above_one() {
        let validator = SeverityValidator;

        let result = validator.validate(&assessment(1.1));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_negative_confidence() {
        let validator = SeverityValidator;

        let result = validator.validate(&assessment(-0.1));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_nan_confidence() {
        let validator = SeverityValidator;

        let result = validator.validate(&assessment(f32::NAN));

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_justification() {
        let validator = SeverityValidator;

        let assessment = SeverityAssessment {
            severity: Severity::High,

            confidence: 0.9,

            justification: "   ".into(),
        };

        let error = validator
            .validate(&assessment)
            .expect_err("empty justification should fail");

        assert!(matches!(error, SeverityValidationError::EmptyJustification));
    }
}
