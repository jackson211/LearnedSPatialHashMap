use std::any::Any;

pub trait Object {
    fn type_name(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub fn type_name(x: &dyn Object) -> &str {
    x.type_name()
}

pub fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}

impl Object for i32 {
    fn type_name(&self) -> &str {
        "i32"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
