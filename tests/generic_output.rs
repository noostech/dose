use dose::{provider, Context};
use fake::{Fake, Faker};
use std::sync::Arc;

mod base;
pub use base::*;

#[provider(singleton = true)]
fn provide_a_trait(_: &Context<Config>) -> Arc<dyn ATrait> {
    Arc::new(AStruct { name: Faker.fake() })
}

#[test]
fn test_generic_outputs_behave_the_same() {
    let mut context = Context::new(Config {});

    let a_trait_1: Arc<dyn ATrait> = context.get();
    let a_trait_2: ATraitRef = context.get();

    assert_eq!(a_trait_1.a_string(), a_trait_2.a_string());
}
