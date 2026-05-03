use crate::{HookStorage, Hookable, StringHook};

/// A hook that trims whitespace from the input string
pub(crate) struct TrimHook {
  pub hook: StringHook,
}
impl_string_hook_storage!(TrimHook);

impl Hookable<String> for TrimHook {
  fn execute(&self, value: String) -> String {
    value.trim().to_string()
  }
}

pub struct UppercaseHook {
  pub hook: StringHook,
}
impl_string_hook_storage!(UppercaseHook);

impl Hookable<String> for UppercaseHook {
  fn execute(&self, value: String) -> String {
    value.to_uppercase()
  }
}

pub struct AppendHook {
  pub hook: StringHook,
}
impl_string_hook_storage!(AppendHook);

impl Hookable<String> for AppendHook {
  fn post_process(&self, value: String) -> String {
    format!("{} {}", value, "✅")
  }
  fn execute(&self, value: String) -> String {
    format!("{}!", value)
  }
}
