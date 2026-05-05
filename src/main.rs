/// Implements the `HookStorage<String>`
/// trait for a given type. This macro generates the required methods to manage
/// `StringHook` storage through immutable and mutable references.
///
/// # Parameters
/// - `$type`: The type for which the `HookStorage<String>` trait will be implemented.
///
/// # Generated Trait Implementation
/// - `fn hook(&self) -> &StringHook`:
///   Returns an immutable reference to the `hook` field of the provided type.
/// - `fn hook_mut(&mut self) -> &mut StringHook`:
///   Returns a mutable reference to the `hook` field of the provided type.
///
/// This macro assumes the type `$type` has a field named `hook` of type `StringHook`.
///
/// # Example
/// ```rust
/// use your_crate::HookStorage;
/// use your_crate::StringHook;
///
/// struct MyStruct {
///     hook: StringHook,
/// }
///
/// impl_string_hook_storage!(MyStruct);
///
/// let mut my_instance = MyStruct { hook: StringHook::new() };
///
/// // Access the hook immutably
/// let hook_ref: &StringHook = my_instance.hook();
///
/// // Access the hook mutably
/// let hook_mut_ref: &mut StringHook = my_instance.hook_mut();
/// ```
macro_rules! impl_string_hook_storage {
  ($type:ty) => {
    impl HookStorage<String> for $type {
      fn hook(&self) -> &StringHook {
        &self.hook
      }

      fn hook_mut(&mut self) -> &mut StringHook {
        &mut self.hook
      }
    }
  };
}
mod hooks;

pub type Hook<T> = Option<Box<dyn Hookable<T>>>;
pub type StringHook = Option<Box<dyn Hookable<String>>>;

pub trait HookStorage<T> {
  fn hook(&self) -> &Hook<T>;
  fn hook_mut(&mut self) -> &mut Hook<T>;
}

/// Trait for hookable objects
pub trait Hookable<T>: HookStorage<T> {
  fn pre_process(&self, value: T) -> (bool, T) {
    (true, value)
  }

  fn post_process(&self, value: T) -> T {
    value
  }

  fn process(&self, value: T) -> T {
    let (should_process, value) = self.pre_process(value);
    if should_process {
      let value = self.execute(value);
      self.post_process(self.process_next(value))
    } else {
      self.post_process(self.process_next(value))
    }
  }

  fn execute(&self, value: T) -> T;

  fn sethook(&mut self, hook: Box<dyn Hookable<T>>) {
    match self.hook_mut() {
      Some(existing_hook) => existing_hook.sethook(hook),
      empty_slot @ None => {
        *empty_slot = Some(hook);
      }
    }
  }

  fn process_next(&self, value: T) -> T {
    match self.hook() {
      Some(next_hook) => next_hook.process(value),
      None => value,
    }
  }
}

fn main() {
  use hooks::utilities::{AppendHook, TrimHook, UppercaseHook, PrintHook};

  let mut hook1 = TrimHook { hook: None };
  let hook2 = AppendHook { hook: None };
  let hook3 = UppercaseHook { hook: None };
  let hook4 = PrintHook { hook: None, name: Some("Print hook 123".to_string()) };
  hook1.sethook(Box::new(hook2));
  hook1.sethook(Box::new(hook4));
  hook1.sethook(Box::new(hook3));

  println!("Result: {}", hook1.process("  hello world  ".to_string()));
}
