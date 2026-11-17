// Integration test for SigmaOS-default-application-manager
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use default_application_manager as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
