use dose::{provider, Context};
use fake::{Fake, Faker};
use log::debug;
use std::sync::Arc;

mod base;
pub use base::*;

#[provider]
fn provide_a_trait(_: &Context<Config>) -> ATraitRef {
    Arc::new(AStruct { name: Faker.fake() })
}

#[test]
fn test_no_singleton_different_instance() {
    let mut context = Context::new(Config {});

    let a_trait_1: ATraitRef = context.get();
    let a_trait_2: ATraitRef = context.get();

    assert_ne!(a_trait_1.a_string(), a_trait_2.a_string());
}
