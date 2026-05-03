/// Our basic hook structure
struct Hook<T> {
    hook: Option<Box<dyn Hookable<T>>>,
}

trait HookStorage<T> {
    fn hook(&self) -> &Option<Box<dyn Hookable<T>>>;
    fn hook_mut(&mut self) -> &mut Option<Box<dyn Hookable<T>>>;
}

/// Trait for hookable objects
trait Hookable<T>: HookStorage<T> {
    fn process(&self, value: T) -> T;

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

macro_rules! impl_hook_storage {
    ($type:ty, $t:ty) => {
        impl HookStorage<$t> for $type {
            fn hook(&self) -> &Option<Box<dyn Hookable<$t>>> {
                &self.hook
            }

            fn hook_mut(&mut self) -> &mut Option<Box<dyn Hookable<$t>>> {
                &mut self.hook
            }
        }
    };
}

fn main() {
    struct TrimHook{
        hook: Option<Box<dyn Hookable<String>>>,
    }

    impl_hook_storage!(TrimHook, String);

    impl Hookable<String> for TrimHook {
        fn process(&self, mut value: String) -> String {
            value = value.trim().to_string();
            self.process_next(value)
        }
    }

    struct AppendHook{
        hook: Option<Box<dyn Hookable<String>>>,
    }

    impl_hook_storage!(AppendHook, String);

    impl Hookable<String> for AppendHook {
        fn process(&self, mut value: String) -> String {
            value.push_str("!");
            self.process_next(value)
        }
    }

    let mut hook1 = TrimHook {hook: None};
    let hook2 = AppendHook {hook: None};
    hook1.sethook(Box::new(hook2));

    println!("Result: {}", hook1.process("  hello ".to_string()));
}
