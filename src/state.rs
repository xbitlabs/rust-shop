use std::ops::Deref;
use std::sync::Arc;
use serde::Serialize;

pub struct State<T: ?Sized>(Arc<T>);

impl<T> State<T> {
    /// Create new `State` instance.
    pub fn new(state: T) -> State<T> {
        State(Arc::new(state))
    }
}

impl<T: ?Sized> State<T> {
    /// Returns reference to inner `T`.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Unwraps to the internal `Arc<T>`
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> Deref for State<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for State<T> {
    fn clone(&self) -> State<T> {
        State(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for State<T> {
    fn from(arc: Arc<T>) -> Self {
        State(arc)
    }
}

impl<T> Serialize for State<T>
    where
        T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
