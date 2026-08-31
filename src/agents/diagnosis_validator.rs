use crate::domain::Diagnosis;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DiagnosisValidationError {
    #[error("probable cause at index {index} has invalid confidence {value}")]
    InvalidConfidence { index: usize, value: f32 },

    #[error("probable cause at index {index} has an empty description")]
    EmptyCauseDescription { index: usize },

    #[error("investigation at index {index} has an empty description")]
    EmptyInvestigation { index: usize },

    #[error("recommendation at index {index} has an empty description")]
    EmptyRecommendation { index: usize },
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DiagnosisValidator;

impl DiagnosisValidator {
    pub fn validate(&self, diagnosis: &Diagnosis) -> Result<(), DiagnosisValidationError> {
        for (index, cause) in diagnosis.probable_causes.iter().enumerate() {
            if cause.description.trim().is_empty() {
                return Err(DiagnosisValidationError::EmptyCauseDescription { index });
            }

            if !cause.confidence.is_finite() || !(0.0..=1.0).contains(&cause.confidence) {
                return Err(DiagnosisValidationError::InvalidConfidence {
                    index,
                    value: cause.confidence,
                });
            }
        }

        for (index, investigation) in diagnosis.investigations.iter().enumerate() {
            if investigation.description.trim().is_empty() {
                return Err(DiagnosisValidationError::EmptyInvestigation { index });
            }
        }

        for (index, recommendation) in diagnosis.recommendations.iter().enumerate() {
            if recommendation.description.trim().is_empty() {
                return Err(DiagnosisValidationError::EmptyRecommendation { index });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Diagnosis, Investigation, ProbableCause, Recommendation};

    use super::*;

    fn valid_diagnosis() -> Diagnosis {
        Diagnosis {
            probable_causes: vec![ProbableCause {
                description: "Possible unauthorized access".into(),
                confidence: 0.7,
            }],

            investigations: vec![Investigation {
                description: "Review authentication logs".into(),
            }],

            recommendations: vec![Recommendation {
                description: "Review account access".into(),
            }],
        }
    }

    #[test]
    fn accepts_valid_diagnosis() {
        let validator = DiagnosisValidator;

        validator
            .validate(&valid_diagnosis())
            .expect("diagnosis should be valid");
    }

    #[test]
    fn rejects_invalid_cause_confidence() {
        let validator = DiagnosisValidator;

        let mut diagnosis = valid_diagnosis();

        diagnosis.probable_causes[0].confidence = 1.5;

        let error = validator
            .validate(&diagnosis)
            .expect_err("confidence should be rejected");

        assert!(matches!(
            error,
            DiagnosisValidationError::InvalidConfidence { index: 0, .. }
        ));
    }

    #[test]
    fn rejects_empty_cause_description() {
        let validator = DiagnosisValidator;

        let mut diagnosis = valid_diagnosis();

        diagnosis.probable_causes[0].description = "   ".into();

        let error = validator
            .validate(&diagnosis)
            .expect_err("empty cause should be rejected");

        assert!(matches!(
            error,
            DiagnosisValidationError::EmptyCauseDescription { index: 0 }
        ));
    }

    #[test]
    fn accepts_diagnosis_without_probable_causes() {
        let validator = DiagnosisValidator;

        let diagnosis = Diagnosis {
            probable_causes: Vec::new(),

            investigations: vec![Investigation {
                description: "Collect additional logs".into(),
            }],

            recommendations: Vec::new(),
        };

        validator
            .validate(&diagnosis)
            .expect("absence of probable causes should be valid");
    }
}
