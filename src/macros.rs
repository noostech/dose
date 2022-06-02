#[macro_export]
macro_rules! init {
    ( ) => {
        pub mod dose_private {
            pub trait Injector<T> {
                fn get(&mut self) -> T;
            }
        }
    };
}

#[macro_export]
macro_rules! get {
    ($context:expr) => {{
        use crate::dose_private::Injector;
        $context.get()
    }};
}
