use std::fs;
use std::path::PathBuf;

use us_core_allergy::validate_allergy_json;

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(name);
    fs::read_to_string(path).expect("fixture")
}

#[test]
fn valid_medication_allergy() {
    let report = validate_allergy_json(&fixture("valid-medication-allergy.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn missing_verification_fails() {
    let report = validate_allergy_json(&fixture("invalid-missing-verification.json")).unwrap();
    assert!(!report.is_valid());
}
