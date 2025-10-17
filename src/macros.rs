#[macro_export]
macro_rules! impl_global {
    ($t:ty) => {
        impl Global for $t {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
