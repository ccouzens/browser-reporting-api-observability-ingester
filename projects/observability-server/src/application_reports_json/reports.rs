use super::Report;
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Reports(
    // Safari 26 sends reports individually without the wrapping array. Whilst not spec
    // conforming, we should be compatible with it.
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
    fn it_works() {
        println!("got here");
        for entry in WalkDir::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("../example-reports/"))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".json")
                && !file_name.ends_with(".parsed-body.json")
                && file_name.ends_with("2026-11-01-site-with-violations-single.json")
            {
                let file = File::open(entry.path()).unwrap();
                let reader = BufReader::new(file);
                let example_report: ExampleReport = serde_json::from_reader(reader).unwrap();
                println!("{:?}", &example_report);
                println!(
                    "{}",
                    serde_json::to_string_pretty(&example_report.body).unwrap()
                );
            }
        }
    }
}
