use std::collections::{HashMap, HashSet};

struct ModelTracker {
    models: HashMap<String, Vec<String>>,
}

impl ModelTracker {
    fn new() -> Self {
        ModelTracker {
            models: HashMap::new(),
        }
    }

    fn add_model(&mut self, model_name: &str, features: Vec<String>) {
        self.models.insert(model_name.to_string(), features);
    }

    fn get_model_features(&self, model_name: &str) -> Option<&Vec<String>> {
        self.models.get(model_name)
    }

    fn has_model(&self, model_name: &str) -> bool {
        self.models.contains_key(model_name)
    }

    fn remove_model(&mut self, model_name: &str) {
        self.models.remove(model_name);
    }
}

fn main() {
    let mut tracker = ModelTracker::new();

    tracker.add_model("Model1", vec!["Feature1".to_string(), "Feature2".to_string()]);
    tracker.add_model("Model2", vec!["Feature3".to_string(), "Feature4".to_string()]);

    assert!(tracker.has_model("Model1"));
    assert!(tracker.has_model("Model2"));

    assert_eq!(
        tracker.get_model_features("Model1").unwrap(),
        &vec!["Feature1".to_string(), "Feature2".to_string()]
    );

    tracker.remove_model("Model1");

    assert!(!tracker.has_model("Model1"));
}