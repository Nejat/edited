#[test]
fn html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

// #[test]
// fn readme_usage_version() {
//     version_sync::assert_contains_regex!("README.md", "<some text here> {version}");
// }
