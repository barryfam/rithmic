pub trait DebugOr {
    fn debug_or(&self, default: impl AsRef<str>) -> String;
}

impl<T: std::fmt::Debug> DebugOr for T
{
    fn debug_or(&self, default: impl AsRef<str>) -> String
    {
        let _ = default;
        format!("{:?}", self)
    }
}

impl<T> DebugOr for T
{
    default fn debug_or(&self, default: impl AsRef<str>) -> String
    {
        default.as_ref().to_string()
    }
}
