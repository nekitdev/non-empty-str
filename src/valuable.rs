#[cfg(not(feature = "valuable"))]
compile_error!("expected `valuable` to be enabled");

use valuable::{Valuable, Value, Visit};

use crate::str::NonEmptyStr;

impl Valuable for &NonEmptyStr {
    fn as_value(&self) -> Value<'_> {
        Value::String(self.as_str())
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.as_value());
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    use valuable::{Valuable, Value, Visit};

    use crate::string::NonEmptyString;

    impl Valuable for NonEmptyString {
        fn as_value(&self) -> Value<'_> {
            Value::String(self.as_str())
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_value(self.as_value());
        }
    }
}
