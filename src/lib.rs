extern crate apify_client;

// TODO: re-export serde from client;
extern crate serde;

pub mod utils;
pub mod actor;
pub mod dataset;

use crate::actor::Actor;

#[cfg(test)]
mod tests {
    use super::*;

    // let name = "RUST-TEST-PUSH-DATA";

    // Simple await macro for tests
    macro_rules! await_test {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    fn dummy_data() -> Vec<serde_json::Value> {
        let item1 = serde_json::json!({ "obj": 1 });
        let item2 = serde_json::json!({ "obj": 2 });
        vec![item1, item2]
    }
    #[test]
    fn push_data_local() {
        let mut actor = Actor::new();
        
        let v = dummy_data();
        await_test!(actor.push_data(&v));
    }

    #[test]
    fn push_data_local_named() {
        let mut actor = Actor::new();
        let dataset_handle = await_test!(actor.openDataset(Some("my-dataset"), false));
        
        let v = dummy_data();
        await_test!(dataset_handle.push_data(&v));
    }

    fn set_fake_cloud_env () -> Actor {
        std::env::set_var("APIFY_IS_AT_HOME", "1");

        let token = std::fs::read_to_string("test/test_token.txt").unwrap();
        std::env::set_var("APIFY_TOKEN", token);

        let mut actor = Actor::new();

        // Create dummy dataset as a default dataset for this test
        let dataset = await_test!(actor.client.create_dataset("RUST-TEST-PUSH-DATA").send()).unwrap();
        std::env::set_var("APIFY_DEFAULT_DATASET_ID", dataset.id);
        actor
    }

    #[test]
    fn push_data_cloud () {
        let mut actor = set_fake_cloud_env();
        
        let v = dummy_data();
        await_test!(actor.push_data(&v));
    }

    #[test]
    fn push_data_cloud_named() {
        let mut actor = set_fake_cloud_env();
        
        let dataset_handle = await_test!(actor.openDataset(Some("my-dataset"), false));
        
        let v = dummy_data();
        await_test!(dataset_handle.push_data(&v));
    }
}
