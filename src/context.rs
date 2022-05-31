use log::debug;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Context<C> {
    pub instances: HashMap<TypeId, Rc<dyn Any + 'static>>,
    pub config: C,
}

pub fn instance_id<R>() -> TypeId
where
    R: 'static,
{
    TypeId::of::<R>()
}

impl<C> Context<C> {
    pub fn new(config: C) -> Self {
        Self {
            instances: HashMap::new(),
            config,
        }
    }

    pub fn resolve_singleton<T, F>(&mut self, func_factory: F, name: &str) -> T
    where
        T: Clone + 'static,
        F: Fn(&mut Context<C>) -> T,
    {
        let instance_id = instance_id::<T>();
        let state = self.resolve_state_singleton(instance_id, func_factory, name);

        with_cast::<T>(state)
    }

    fn resolve_state_singleton<F, T>(
        &mut self,
        key: TypeId,
        create_func: F,
        name: &str,
    ) -> Rc<dyn Any + 'static>
    where
        F: Fn(&mut Context<C>) -> T,
        T: 'static,
    {
        let result = self.instances.get(&key);
        match result {
            Some(val) => {
                debug!("Global type '{}' found, cloning same instance.", name);
                val.clone()
            }
            None => {
                debug!("Global type '{}' not found, creating new instance.", name);
                let instance = create_func(self);
                let instance = Rc::new(instance);
                self.instances.insert(key, instance.clone());
                instance
            }
        }
    }
}

fn with_cast<T>(obj: Rc<dyn Any + 'static>) -> T
where
    T: Clone + 'static,
{
    let val = match obj.downcast::<T>() {
        Ok(val) => val,
        Err(err) => panic!("Internal error, unable to cast {:?}", err),
    };
    val.as_ref().clone()
}
