use std::fmt::Debug;
use std::hash::Hash;

pub trait EventKey: Debug + Hash + PartialEq + Eq +  Copy + Send + Sync  + 'static {}

impl<T> EventKey for T where T: Debug + Hash + PartialEq + Eq +  Copy + Send + Sync + 'static {}
