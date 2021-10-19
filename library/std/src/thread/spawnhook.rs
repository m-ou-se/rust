use crate::sys_common::mutex::StaticMutex;

static SPAWN_HOOKS_MUTEX: StaticMutex = StaticMutex::new();
static mut SPAWN_HOOKS: Vec<Box<dyn Fn() -> Box<dyn FnOnce()>>> = Vec::new();

/// TODO
#[unstable(feature = "thread_spawn_hook", issue = "none")]
pub fn add_spawn_hook(hook: Box<dyn Fn() -> Box<dyn FnOnce()>>) {
    unsafe {
        let _guard = SPAWN_HOOKS_MUTEX.lock();
        SPAWN_HOOKS.push(hook);
    }
}

pub(super) fn run_spawn_hooks() -> Vec<Box<dyn FnOnce()>> {
    unsafe {
        let _guard = SPAWN_HOOKS_MUTEX.lock();
        SPAWN_HOOKS.iter().map(|f| f()).collect()
    }
}
