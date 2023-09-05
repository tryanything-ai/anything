use super::delegate::{Delegate, Response, Subscription};
use std::{fmt::Debug, ops::Deref};

/// Wrapper which owns a value and executes callbacks
///
/// ```rust
/// use anything_events::{Observable, Delegate, Response};
///
/// let mut var = Observable::new(10);
/// var.subscribe(|new_var| {
///     println!("variable is now: {new_var}");
///     Response::Continue
/// });
/// var.mutate(|v| {
///     *v += 10; // prints "variable is now 20"
/// });
/// ```
///
/// Observable implements `Deref` so the value can be retrieved through `*variable`
#[derive(Debug)]
pub struct Observable<'a, T> {
    value: T,
    delegate: Delegate<'a, T>,
}

impl<'o, T> Observable<'o, T> {
    /// Create a new obserable with initial value
    ///
    /// ```rust
    /// use anything_events::Observable;
    /// let name = Observable::new(String::from("Ari"));
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            value,
            delegate: Delegate {
                subscriptions: Default::default(),
            },
        }
    }

    /// Creates a subscription callback when value contained in observable is mutated with `mutate()`
    ///
    /// ```rust
    /// use anything_events::{Observable, Response};
    ///
    /// let mut name = Observable::new("Ari");
    ///     name.subscribe(|name| {
    ///     Response::Continue
    /// });
    /// ```
    pub fn subscribe<C: FnMut(&T) -> Response + 'o + Send>(&self, callback: C) -> Subscription {
        self.delegate.subscribe(callback)
    }

    /// Remove a previously subscribed callback
    ///
    /// ```rust
    /// use anything_events::{Observable, Response};
    ///
    /// let mut name = Observable::new("Ari");
    /// let subscription = name.subscribe(|name| {
    ///     Response::Continue
    /// });
    /// name.unsubscribe(subscription);
    /// ```
    pub fn unsubscribe(&self, subscription: Subscription) {
        self.delegate.unsubscribe(subscription);
    }

    /// Get the reference of a delegate that executes subscription functions
    /// when observable is mutated. Use this when creating a struct with an
    /// observable member where users should have access to the value
    /// through subscription
    ///
    /// ```rust
    /// use anything_events::{Delegate, Observable};
    ///
    /// struct User<'a> {
    ///     name: Observable<'a, String>,
    /// }
    ///
    /// impl<'a> User<'a> {
    ///     pub fn delegate(&self) -> &Delegate<'a, String> {
    ///         self.name.delegate()
    ///     }
    /// }
    /// ```
    pub fn delegate(&self) -> &Delegate<'o, T> {
        &self.delegate
    }

    /// Run a function which *can* mutate the observable value.
    /// Subscription callbacks will be executed regardless of the contents
    /// of the `mutateion` function
    ///
    /// ```rust
    /// use anything_events::Observable;
    ///
    /// let mut name = Observable::new(String::from("Ari"));
    /// name.mutate(|n| n.push_str(" "));
    /// name.mutate(|n| n.push_str("Rock"));
    /// name.mutate(|n| n.push_str("Star"));
    ///
    /// assert_eq!(name.as_str(), "Ari RockStar");
    /// ```
    pub fn mutate<M>(&mut self, mutation: M)
    where
        M: FnOnce(&mut T),
    {
        mutation(&mut self.value);
        self.delegate.broadcast(&self.value);
    }
}

impl<T> Default for Observable<'_, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

/// Allows the observable value to be fetched through derefencing
impl<T> Deref for Observable<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
