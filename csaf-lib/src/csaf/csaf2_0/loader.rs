use super::schema::CommonSecurityAdvisoryFramework;
use std::{fs::File, io::BufReader};

pub fn load_document(path: &str) -> std::io::Result<CommonSecurityAdvisoryFramework> {
    println!("Trying to load document {}", path);

    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let doc: CommonSecurityAdvisoryFramework = serde_json::from_reader(reader)?;
    println!(
        "Successfully parsed document '{}'",
        doc.document.title.to_string()
    );

    Ok(doc)
}

#[cfg(test)]
mod tests {
    use crate::csaf::csaf2_0::schema::{
        CategoryOfPublisher, CommonSecurityAdvisoryFramework, DocumentLevelMetaData, Publisher,
        Revision, Tracking,
    };

    fn mock_document() -> CommonSecurityAdvisoryFramework {
        let now = chrono::Utc::now().to_string();
        let metadata: DocumentLevelMetaData = DocumentLevelMetaData::builder()
            .title("Test")
            .category("csaf_base")
            .csaf_version("2.0")
            .publisher(
                Publisher::builder()
                    .category(CategoryOfPublisher::Coordinator)
                    .name("test")
                    .namespace("http://example.com"),
            )
            .tracking(
                Tracking::builder()
                    .id("test")
                    .current_release_date(now.clone())
                    .initial_release_date(now.clone())
                    .status("final")
                    .version("1")
                    .revision_history(vec![Revision::builder()
                        .number("1")
                        .date(now.clone())
                        .summary("test")
                        .try_into()
                        .unwrap()]),
            )
            .try_into()
            .unwrap();
        CommonSecurityAdvisoryFramework::builder()
            .document(metadata)
            .try_into()
            .unwrap()
    }

    #[test]
    fn it_works() {
        mock_document();
    }
}
