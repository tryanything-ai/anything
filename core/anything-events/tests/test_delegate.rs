use anything_events::{Delegate, Response};
use parking_lot::{Mutex, ReentrantMutex, RwLock};
use std::{cell::RefCell, ops::Deref, sync::Arc};

#[test]
fn test_simple_subscription() {
    let mut call_count = 0;
    {
        let d = Delegate::new();
        d.subscribe(|_| {
            call_count += 1;
            Response::Continue
        });
        d.notify();
        d.notify();
        d.notify();
    }
    assert_eq!(call_count, 3);
}

#[test]
fn test_no_update_unsubscribed_callbacks() {
    let mut called = 0;
    {
        let d = Delegate::new();
        let sub = d.subscribe(|_| {
            called += 1;
            Response::Continue
        });
        d.unsubscribe(sub);
        d.notify();
    }
    assert_eq!(0, called);
}

#[test]
fn test_cannot_unsubscribe_from_non_self_delegate() {
    let mut called = 0;
    {
        let d = Delegate::<()>::new();
        let d2 = Delegate::<()>::new();
        let _ = d.subscribe(|_| {
            called += 1;
            Response::Continue
        });
        let s2 = d2.subscribe(|_| Response::Continue);
        d.unsubscribe(s2);
        d.notify();
    }
    assert_eq!(called, 1);
}

#[test]
fn unsubscribing_within_callback_is_noop() {
    let d = Arc::new(ReentrantMutex::new(Delegate::new()));
    let call_count = Arc::new(Mutex::new(RefCell::new(0)));
    let subscription = Arc::new(Mutex::new(RefCell::new(None)));

    let d_clone = d.clone();
    let call_count_clone = call_count.clone();
    let subscription_clone = subscription.clone();

    subscription
        .lock()
        .replace(Some(d.lock().subscribe(move |_| {
            let old_count = *call_count_clone.lock().borrow();
            *call_count_clone.lock().borrow_mut() = old_count + 1;
            if let Some(subscription) = subscription_clone.lock().deref().borrow_mut().take() {
                d_clone.lock().unsubscribe(subscription);
            }
            Response::Continue
        })));

    d.lock().notify();
    d.lock().notify();
    assert_eq!(*call_count.lock().borrow(), 2);
}

#[test]
fn test_multiple_callbacks_execute_in_order() {
    let d = Arc::new(ReentrantMutex::new(Delegate::<String>::new()));
    let name = Arc::new(RwLock::new(String::from("Hello")));

    {
        let d_clone = d.clone();
        let name_clone = name.clone();
        let name_clone2 = name.clone();
        let name_clone3 = name.clone();

        let cb1 = move |_new_name: &String| {
            // let mut name_write = name.clone().write();
            // name_write.push_str("bob");
            let mut name_write = name_clone.write();
            name_write.push_str(" bob");
            Response::Continue
        };
        let cb2 = move |_new_name: &String| {
            let mut name_write = name_clone2.write();
            name_write.push_str(" anne");
            Response::Continue
        };
        let cb3 = move |new_name: &String| {
            let mut name_write = name_clone3.write();
            name_write.push_str(&new_name);
            Response::Continue
        };
        d_clone.lock().subscribe(cb1);
        d_clone.lock().subscribe(cb2);
        d_clone.lock().subscribe(cb3);

        // The value doesn't matter, we're not using it
        d_clone.lock().broadcast(String::from(" joe"));
    }
    assert_eq!(name.read().as_str(), "Hello bob anne joe");
}
