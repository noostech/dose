use std::sync::Arc;

mod base;
pub use base::*;
#[derive(Clone)]
struct AnotherStruct {
    a: Arc<dyn ATrait>,
}
mod nested {
    use super::{AStruct, ATrait, ATraitRef, AnotherStruct};
    use crate::Config;
    use dose::{get, provider, Context};
    use fake::{Fake, Faker};
    use std::sync::Arc;

    #[provider(singleton = true)]
    fn provide_a_struct(context: &mut Context<Config>) -> AnotherStruct {
        AnotherStruct { a: get!(context) }
    }

    #[provider(singleton = true)]
    fn provide_a_string(_: &Context<Config>) -> String {
        String::from("")
    }

    #[provider(singleton = true)]
    fn provide_a_trait(_: &Context<Config>) -> Arc<dyn ATrait> {
        Arc::new(AStruct { name: Faker.fake() })
    }

    #[test]
    fn test_can_provide_a_struct() {
        let mut context = Context::new(Config {});

        let a_trait: ATraitRef = get!(context);
        let a_struct: AnotherStruct = get!(context);

        assert_eq!(a_trait.a_string(), a_struct.a.a_string());
    }
}
