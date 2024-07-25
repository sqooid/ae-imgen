pub trait EnumMethods<T> {
    fn get_arg<S: Into<usize>>(&self, i: S) -> &T;
    fn set_arg<S: Into<usize>>(&self, i: S, value: T);
}
