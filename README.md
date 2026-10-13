# US Core AllergyIntolerance (FHIR R4) — Rust

Small library and CLI that sanity-check **US Core AllergyIntolerance** JSON before it leaves your integration layer: clinical and verification status bindings, **RxNorm** or **SNOMED** on `code`, `patient`, and **reaction.manifestation**.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 05** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [US Core AllergyIntolerance](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-allergyintolerance.html)

## What it checks

- `meta.profile` includes [US Core AllergyIntolerance STU 6.1](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-allergyintolerance.html)
- `clinicalStatus` uses `http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical`
- `verificationStatus` uses `http://terminology.hl7.org/CodeSystem/allergyintolerance-verification`
- `type` is `allergy` or `intolerance`; warnings if `category` missing
- Medication allergies need **RxNorm** or **SNOMED** on `code`
- `patient.reference` required; warnings if `reaction` missing; each reaction needs `manifestation`

| Problem | What breaks |
|--------|-------------|
| **NKA copied from legacy tables** | Missing verification status or empty reactions |
| **Medication allergy without RxNorm** | Pharmacy code not mapped to RxNorm/SNOMED |
| **No patient link** | Orphan allergy rows in bulk export |
| **Negation not modeled** | “No known allergies” sent as empty list vs proper negation |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-allergy -- examples/valid-medication-allergy.json
cargo run --bin validate-allergy -- examples/invalid-missing-verification.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-allergy -- examples/valid-medication-allergy.json
```

**Pass** (`examples/valid-medication-allergy.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-missing-verification.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "AllergyIntolerance.verificationStatus",
    "detail": "verificationStatus must use http://terminology.hl7.org/CodeSystem/allergyintolerance-verification",
    "rule_id": "verification-status-binding"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken payloads for tests and demos

## Inferno vs this repo

Inferno exercises your **API** against CMS-style scenarios. This repo is about **payload shape** — allergy lists that fail Must Support and binding rules in production.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
