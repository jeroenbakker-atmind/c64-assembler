/// Trait for user counts of objects.
pub trait UserCount {
    /// Add a user
    fn user_increase(&mut self);

    /// How many users have been registered
    fn user_count(&self) -> usize;

    /// Are there any users registered
    fn user_empty(&self) -> bool {
        self.user_count() == 0
    }
}
