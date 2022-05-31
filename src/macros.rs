#[macro_export]
macro_rules! init {
    ( ) => {
        pub trait Injector<T> {
            fn get(&mut self) -> T;
        }
    };
}
