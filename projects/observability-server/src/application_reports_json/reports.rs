use super::Report;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Reports(
    // Safari 26 sends reports individually without the wrapping array. Whilst
    // not spec conforming, we should be compatible with it.
    #[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<Report>,
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader, path::Path};
    use walkdir::WalkDir;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ExampleReport {
        body: Reports,
    }

    #[test]
    fn it_parses_captured_examples() {
        for entry in WalkDir::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("../example-reports/"))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".json") {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::from_reader::<_, ExampleReport>(BufReader::new(
                            File::open(entry.path()).unwrap()
                        ))
                        .unwrap()
                        .body
                    )
                    .unwrap()
                );
            }
        }
    }
}
