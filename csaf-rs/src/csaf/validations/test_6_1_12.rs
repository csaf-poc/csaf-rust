use crate::csaf::generated::language_subtags::is_valid_language_subtag;
use crate::csaf::getter_traits::{CsafTrait, DocumentTrait};
use crate::csaf::validation::ValidationError;

pub fn test_6_1_12_language(doc: &impl CsafTrait) -> Result<(), ValidationError> {
    let document = doc.get_document();

    // Check /document/lang if it exists
    if let Some(lang) = document.get_lang() {
        validate_language_code(lang, "/document/lang")?;
    }

    // Check /document/source_lang if it exists  
    if let Some(source_lang) = document.get_source_lang() {
        validate_language_code(source_lang, "/document/source_lang")?;
    }

    Ok(())
}

fn validate_language_code(lang_code: &str, json_path: &str) -> Result<(), ValidationError> {
    // Extract the primary language subtag (everything before the first hyphen)
    let primary_subtag = lang_code.split('-').next().unwrap_or(lang_code);

    if !is_valid_language_subtag(primary_subtag) {
        return Err(ValidationError {
            message: format!("Invalid language code '{}': primary language subtag '{}' is not a valid language subtag", lang_code, primary_subtag),
            instance_path: json_path.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::csaf::test_helper::{run_csaf20_tests, run_csaf21_tests};
    use crate::csaf::validation::ValidationError;
    use crate::csaf::validations::test_6_1_12::test_6_1_12_language;
    use std::collections::HashMap;

    #[test]
    fn test_test_6_1_12() {
        let error01 = ValidationError {
            message: "Invalid language code 'EZ': primary language subtag 'EZ' is not a valid language subtag".to_string(),
            instance_path: "/document/lang".to_string(),
        };
        let errors = HashMap::from([
            ("01", &error01)
        ]);
        run_csaf20_tests("12", test_6_1_12_language, &errors);
        run_csaf21_tests("12", test_6_1_12_language, &errors);
    }
}