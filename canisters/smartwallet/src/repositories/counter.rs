use crate::context::STATE;

pub fn get_counter() -> u128 {
    STATE.with(|s| *s.borrow().counter.get())
}

pub(crate) fn increment_one() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let current_counter = *state.counter.get();
        let _ = state.counter.set(current_counter + 1);
    })
}
