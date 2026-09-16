use tempfile::tempdir;
use url::Url;

use pbuilder::fetch::fetch;

#[tokio::test]
async fn fetches_a_file() {
    let destination = tempdir().unwrap();

    let url = Url::parse(
        "https://www.rust-lang.org/static/images/rust-logo-blk.svg"
    ).unwrap();

    let path = fetch(url, false).unwrap();

    assert!(path.exists());
    assert!(path.metadata().unwrap().len() > 0);
}
