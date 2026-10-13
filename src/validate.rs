use serde_json::Value;

use crate::outcome::{ValidationIssue, ValidationReport};
use crate::profile::{
    ALLERGY_CLINICAL_SYSTEM, ALLERGY_VERIFICATION_SYSTEM, RXNORM_SYSTEM, SNOMED_SYSTEM,
    US_CORE_ALLERGY_PROFILE,
};

pub fn validate_allergy_json(json: &str) -> Result<ValidationReport, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {e}"))?;
    Ok(validate_allergy_value(&value))
}

pub fn validate_allergy_value(ai: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    if ai.get("resourceType").and_then(Value::as_str) != Some("AllergyIntolerance") {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.resourceType",
            "resourceType must be AllergyIntolerance",
            "allergy-resource-type",
        ));
        return report;
    }

    check_meta_profile(ai, &mut report);
    check_clinical_status(ai, &mut report);
    check_verification_status(ai, &mut report);
    check_type_and_category(ai, &mut report);
    check_code(ai, &mut report);
    check_patient(ai, &mut report);
    check_reaction(ai, &mut report);

    report
}

fn check_meta_profile(ai: &Value, report: &mut ValidationReport) {
    let ok = ai
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .any(|p| p == US_CORE_ALLERGY_PROFILE)
        });
    if !ok {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.meta.profile",
            format!("meta.profile should include {}", US_CORE_ALLERGY_PROFILE),
            "us-core-meta-profile",
        ));
    }
}

fn coding_in(system: &str, codings: &Value) -> bool {
    codings.as_array().is_some_and(|arr| {
        arr.iter()
            .any(|c| c.get("system").and_then(Value::as_str) == Some(system))
    })
}

fn check_clinical_status(ai: &Value, report: &mut ValidationReport) {
    let codings = ai.pointer("/clinicalStatus/coding");
    if codings.is_none() || !coding_in(ALLERGY_CLINICAL_SYSTEM, codings.unwrap()) {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.clinicalStatus",
            format!("clinicalStatus must use {}", ALLERGY_CLINICAL_SYSTEM),
            "clinical-status-binding",
        ));
    }
}

fn check_verification_status(ai: &Value, report: &mut ValidationReport) {
    let codings = ai.pointer("/verificationStatus/coding");
    if codings.is_none() || !coding_in(ALLERGY_VERIFICATION_SYSTEM, codings.unwrap()) {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.verificationStatus",
            format!("verificationStatus must use {}", ALLERGY_VERIFICATION_SYSTEM),
            "verification-status-binding",
        ));
    }
}

fn check_type_and_category(ai: &Value, report: &mut ValidationReport) {
    let t = ai.get("type").and_then(Value::as_str);
    if t != Some("allergy") && t != Some("intolerance") {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.type",
            "type should be allergy or intolerance",
            "type-required",
        ));
    }

    let cat = ai.get("category").and_then(Value::as_array);
    if cat.is_none() || cat.is_some_and(|a| a.is_empty()) {
        report.push(ValidationIssue::warning(
            "AllergyIntolerance.category",
            "category (food, medication, environment, biologic) is expected for US exchanges",
            "category-recommended",
        ));
    }
}

fn check_code(ai: &Value, report: &mut ValidationReport) {
    let codings = ai.pointer("/code/coding").and_then(Value::as_array);
    let has_snomed = codings.is_some_and(|arr| {
        arr.iter()
            .any(|c| c.get("system").and_then(Value::as_str) == Some(SNOMED_SYSTEM))
    });
    let has_rxnorm = codings.is_some_and(|arr| {
        arr.iter()
            .any(|c| c.get("system").and_then(Value::as_str) == Some(RXNORM_SYSTEM))
    });

    let cat_medication = ai
        .get("category")
        .and_then(Value::as_array)
        .is_some_and(|cats| cats.iter().any(|c| c.as_str() == Some("medication")));

    if cat_medication && !has_rxnorm && !has_snomed {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.code",
            "medication allergies need RxNorm or SNOMED on code",
            "medication-allergy-code",
        ));
    } else if !has_snomed && !has_rxnorm {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.code",
            "code needs SNOMED or RxNorm coding",
            "allergy-code-required",
        ));
    }
}

fn check_patient(ai: &Value, report: &mut ValidationReport) {
    if ai
        .pointer("/patient/reference")
        .and_then(Value::as_str)
        .is_none_or(|r| r.is_empty())
    {
        report.push(ValidationIssue::error(
            "AllergyIntolerance.patient",
            "patient.reference is required",
            "patient-required",
        ));
    }
}

fn check_reaction(ai: &Value, report: &mut ValidationReport) {
    let reactions = ai.get("reaction").and_then(Value::as_array);
    if reactions.is_none() || reactions.is_some_and(|r| r.is_empty()) {
        report.push(ValidationIssue::warning(
            "AllergyIntolerance.reaction",
            "reaction is Must Support when documenting clinical allergies",
            "reaction-recommended",
        ));
        return;
    }

    for (i, reaction) in reactions.unwrap().iter().enumerate() {
        let manifestations = reaction.get("manifestation").and_then(Value::as_array);
        if manifestations.is_none() || manifestations.is_some_and(|m| m.is_empty()) {
            report.push(ValidationIssue::error(
                format!("AllergyIntolerance.reaction[{}].manifestation", i),
                "each reaction should list manifestation codes",
                "reaction-manifestation",
            ));
        }
    }
}
