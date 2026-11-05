#![allow(clippy::unwrap_used, clippy::expect_used)] // integration tests are not shipped code
use proptest::prelude::*;
use scribetrace_core::domain::PaperId;

proptest! {
    #[test]
    fn valid_ids_never_contain_path_separators(s in ".{0,200}") {
        if let Ok(id) = PaperId::new(s) {
            prop_assert!(!id.as_str().contains('/'));
            prop_assert!(!id.as_str().contains('\\'));
            prop_assert!(!id.as_str().contains('.'));
        }
    }

    #[test]
    fn well_formed_ids_roundtrip(s in "[A-Za-z0-9_-]{1,128}") {
        let id = PaperId::new(s.clone()).unwrap();
        prop_assert_eq!(id.as_str(), s.as_str());
    }
}
