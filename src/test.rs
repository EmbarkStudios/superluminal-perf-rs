use super::*;

#[test]
fn simple() {
    begin_event(b"test1\0");
    begin_event(b"test2\0");
    end_event();
    end_event();
}

#[test]
#[should_panic]
#[cfg(windows)]
pub fn invalid_str() {
    begin_event(b"test1");
    begin_event(b"test1\0aha");
    begin_event(b"test1\0aha\0");
}

#[test]
fn thread() {
    set_current_thread_name("my test thread").unwrap();
    begin_event(b"test1\0");
    end_event();
}
