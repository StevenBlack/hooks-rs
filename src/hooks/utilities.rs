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

/// A hook that uppercases the input string
pub struct UppercaseHook {
  pub hook: StringHook,
}
impl_string_hook_storage!(UppercaseHook);

impl Hookable<String> for UppercaseHook {
  fn execute(&self, value: String) -> String {
    value.to_uppercase()
  }
}

/// A hook that appends a checkmark and exclamation to the input string
pub struct AppendHook {
  pub hook: StringHook,
}
impl_string_hook_storage!(AppendHook);

impl Hookable<String> for AppendHook {
  fn post_process(&self, value: String) -> String {
    format!("{} {}", value, "(✅ added in post-process)")
  }
  fn execute(&self, value: String) -> String {
    format!("{}!", value)
  }
}

/// A hook that prints the currently hooked string to stdout
pub struct PrintHook {
  pub hook: StringHook,
  pub name: Option<String>,
}
impl_string_hook_storage!(PrintHook);
impl Hookable<String> for PrintHook {
  fn execute(&self, value: String) -> String {
    let name = self.name.as_ref().unwrap_or(&"PrintHook".to_string()).to_owned();
    println!("{}: {}", name, value);
    value
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
