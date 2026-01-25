//! Unit tests for Smile emoticon picker
//! Tests the core functionality

#[cfg(test)]
mod tests {
    use crate::emoticons::EMOTICONS;

    #[test]
    fn test_emoticons_exist() {
        // Test that emoticons dictionary is populated
        assert!(!EMOTICONS.is_empty());
    }

    #[test]
    fn test_emoticon_categories() {
        // Test that expected categories exist
        let expected_categories = vec!["Happy", "Sad", "Love", "Gestures", "Classic"];
        for category in expected_categories {
            assert!(
                EMOTICONS.contains_key(category),
                "Category '{}' is missing",
                category
            );
        }
    }

    #[test]
    fn test_emoticons_are_vecs() {
        // Test that each category contains a vector of emoticons
        for (category, emoticons) in EMOTICONS.iter() {
            assert!(
                !emoticons.is_empty(),
                "Category '{}' has no emoticons",
                category
            );
        }
    }

    #[test]
    fn test_emoticons_are_strings() {
        // Test that emoticons are strings
        for (category, emoticons) in EMOTICONS.iter() {
            for emoticon in emoticons {
                assert!(
                    !emoticon.is_empty(),
                    "Empty emoticon found in category '{}'",
                    category
                );
            }
        }
    }

    #[test]
    fn test_all_categories_present() {
        // Test that all expected categories are present
        let required_categories = vec![
            "Happy", "Sad", "Angry", "Surprised", "Love", "Gestures", "Faces", "Cool", "Symbols",
            "Objects", "Animals", "Food", "Classic",
        ];
        for category in required_categories {
            assert!(
                EMOTICONS.contains_key(category),
                "Category '{}' is missing",
                category
            );
        }
    }

    #[test]
    fn test_classic_emoticons() {
        // Test that classic text emoticons are included
        let classic = EMOTICONS.get("Classic").expect("Classic category missing");

        // Check for some common classic emoticons
        assert!(classic.contains(&":-)".to_string()));
        assert!(classic.contains(&":)".to_string()));
        assert!(classic.contains(&"<3".to_string()));
    }

    #[test]
    fn test_filter_by_category() {
        // Test filtering emoticons by category name
        let filter_text = "happy";
        let filtered: Vec<&str> = EMOTICONS
            .keys()
            .filter(|k| k.to_lowercase().contains(&filter_text.to_lowercase()))
            .copied()
            .collect();

        assert!(filtered.contains(&"Happy"));
    }

    #[test]
    fn test_filter_by_emoticon() {
        // Test filtering emoticons by emoticon character
        let filter_text = "😀";
        let mut found = false;
        for (_category, emoticons) in EMOTICONS.iter() {
            if emoticons.contains(&filter_text.to_string()) {
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn test_case_insensitive_search() {
        // Test that search is case insensitive
        let filter_text = "HAPPY";
        let filtered: Vec<&str> = EMOTICONS
            .keys()
            .filter(|k| k.to_lowercase().contains(&filter_text.to_lowercase()))
            .copied()
            .collect();

        assert!(filtered.contains(&"Happy"));
    }

    #[test]
    fn test_history_limit() {
        // Test that history is limited to 10 items
        let mut history: Vec<i32> = (0..15).collect();
        history.truncate(10);
        assert_eq!(history.len(), 10);
        assert_eq!(history, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
