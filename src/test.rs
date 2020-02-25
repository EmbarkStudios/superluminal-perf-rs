use super::*;

#[test]
fn simple() {
    begin_event("test1");
    begin_event("test2");
    end_event();
    end_event();
}

#[test]
fn thread() {
    set_current_thread_name("my test thread");
    begin_event("test1");
    end_event();
}
