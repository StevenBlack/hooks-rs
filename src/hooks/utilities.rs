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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Hookable;

  #[test]
  fn trim_hook_removes_whitespace() {
    let hook = TrimHook { hook: None };
    assert_eq!(hook.process("  hello world  ".to_string()), "hello world");
  }

  #[test]
  fn uppercase_hook_converts_to_uppercase() {
    let hook = UppercaseHook { hook: None };
    assert_eq!(hook.process("hello world".to_string()), "HELLO WORLD");
  }

  #[test]
  fn append_hook_appends_exclamation_and_checkmark() {
    let hook = AppendHook { hook: None };
    assert_eq!(hook.process("hello".to_string()), "hello! ✅");
  }
}
