use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    hash::Hash,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_SUBSCRIPTION_ID: AtomicU64 = AtomicU64::new(0);

type BoxedCallback<'a, T> = Box<dyn FnMut(&T) -> Response + 'a + Send>;
type SubscriptionId = u64;

pub enum Response {
    Continue,
    Cancel,
}

#[derive(Default)]
pub struct Delegate<'d, T> {
    pub(crate) subscriptions: RefCell<HashMap<SubscriptionId, BoxedCallback<'d, T>>>,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Subscription {
    id: SubscriptionId,
}

impl<'d, T> Delegate<'d, T> {
    pub fn new() -> Self {
        Self {
            subscriptions: RefCell::new(HashMap::new()),
        }
    }

    pub fn subscribe<C: FnMut(&T) -> Response + 'd + Send>(&self, callback: C) -> Subscription {
        let id = NEXT_SUBSCRIPTION_ID.fetch_add(1, Ordering::SeqCst);
        let subscription = Subscription { id };
        self.subscriptions
            .borrow_mut()
            .insert(subscription.id, Box::new(callback));
        subscription
    }

    pub fn unsubscribe(&self, subscription: Subscription) {
        self.subscriptions.borrow_mut().remove(&subscription.id);
    }

    pub fn broadcast<U: Borrow<T>>(&self, value: U) {
        let mut notify_subs = self
            .subscriptions
            .borrow()
            .keys()
            .copied()
            .collect::<Vec<_>>();
        notify_subs.sort();

        notify_subs.into_iter().for_each(|sub| {
            let (_, mut callback) = self.subscriptions.borrow_mut().remove_entry(&sub).unwrap();
            match callback(value.borrow()) {
                Response::Continue => {
                    self.subscriptions.borrow_mut().insert(sub, callback);
                }
                Response::Cancel => (),
            }
        });
    }
}

impl Delegate<'_, ()> {
    pub fn notify(&self) {
        self.broadcast(());
    }
}

impl<T> Debug for Delegate<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Delegate")
            .field(
                "subscriptions",
                &format_args!("{} active subscriptions", self.subscriptions.borrow().len()),
            )
            .finish()
    }
}
