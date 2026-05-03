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
  use hooks::utilities::{AppendHook, TrimHook};

  let mut hook1 = TrimHook { hook: None };
  let hook2 = AppendHook { hook: None };
  hook1.sethook(Box::new(hook2));

  println!("Result: {}", hook1.process("  hello world  ".to_string()));
}
