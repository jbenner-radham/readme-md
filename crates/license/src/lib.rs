use std::path::Path;

fn read_general_license_filenames() -> Vec<String> {
    let filenames = ["LICENSE", "LICENSE.md", "LICENSE.txt"];

    filenames
        .iter()
        .filter(|filename| Path::new(filename).is_file())
        .map(|filename| filename.to_string())
        .collect()
}

pub fn get_license_section_body(spdx_identifier: &str) -> String {
    if spdx_identifier.contains(" AND ") {
        return format!("The {spdx_identifier} Licenses. See the license files for details");
    }

    if spdx_identifier.contains(" OR ") {
        return format!("The {spdx_identifier} License. See the license files for details");
    }

    if spdx_identifier.to_uppercase() == "UNLICENSED" {
        return "This is unlicensed proprietary software.".to_string();
    }

    let licenses = read_general_license_filenames();

    if licenses.len() == 1 {
        let license = licenses[0].as_str();

        return format!(
            "The {spdx_identifier} License. See the [license file]({license}) for details."
        );
    }

    format!("The {spdx_identifier} License.")
}
