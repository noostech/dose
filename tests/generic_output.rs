use dose::{get, provider, Context};
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

    let a_trait_1: Arc<dyn ATrait> = get!(context);
    let a_trait_2: ATraitRef = get!(context);

    assert_eq!(a_trait_1.a_string(), a_trait_2.a_string());
}
