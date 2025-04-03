use pgrx::prelude::*;
use pgrx::*;
use std::ptr::addr_of_mut;
use rand::Rng;
use std::{thread, time::Duration};

::pgrx::pg_module_magic!();

#[pg_guard]
pub extern "C" fn _PG_init() {
    unsafe {
        register_hook(&mut HOOKS);
    }
}

fn main() {
    println!("Woke up!");
}

struct Hooks;
static mut HOOKS: Hooks = Hooks;

#[allow(deprecated)]
impl PgHooks for Hooks {
    fn executor_start(
        &mut self,
        query_desc: PgBox<pg_sys::QueryDesc>,
        eflags: i32,
        prev_hook: fn(PgBox<pg_sys::QueryDesc>, i32) -> HookResult<()>,
    ) -> HookResult<()> {
    	let slothNapTime = rand::thread_rng().gen_range(100..10000);
    	notice!("Time for a nap! pgsloth is sleeping for {} ms", slothNapTime);
        thread::sleep(Duration::from_millis(slothNapTime));
        prev_hook(query_desc, eflags)
    }

}

