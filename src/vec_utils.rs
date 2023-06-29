use std::collections::VecDeque;

/// Trait for prepending a slice to a vector creating new Self
pub trait VecPrepend<T> {
    /// Prepend a slice to a vector
    ///
    /// # Examples
    /// ```
    /// use rust_utils::vec_utils::VecPrepend;
    ///
    /// let v = vec![1, 2, 3];
    /// let new = v.prepend(&[4, 5, 6]);
    /// assert_eq!(v, vec![1, 2, 3]);
    /// assert_eq!(new, vec![4, 5, 6, 1, 2, 3]);
    /// ```
    fn prepend(&self, s: &[T]) -> Self
    where
        T: Clone;
}

/// Trait for prepending a slice to a vector
pub trait VecPrependMut<T> {
    /// Prepend a slice to a vector
    ///
    /// # Examples
    /// ```
    /// use rust_utils::vec_utils::VecPrependMut;
    ///
    /// let mut v = vec![1, 2, 3];
    /// v.prepend_mut(&[4, 5, 6]);
    /// assert_eq!(v, vec![4, 5, 6, 1, 2, 3]);
    /// ```
    fn prepend_mut(&mut self, s: &[T])
    where
        T: Clone;
}

impl<T> VecPrepend<T> for Vec<T> {
    fn prepend(&self, s: &[T]) -> Self
    where
        T: Clone,
    {
        let mut v = Vec::with_capacity(s.len() + self.len());
        v.extend_from_slice(s);
        v.extend_from_slice(self);
        v
    }
}

impl<T> VecPrepend<T> for VecDeque<T> {
    fn prepend(&self, s: &[T]) -> Self
    where
        T: Clone,
    {
        let mut v = VecDeque::with_capacity(s.len() + self.len());
        v.extend(s.iter().cloned());
        v.extend(self.iter().cloned());
        v
    }
}

impl<T> VecPrependMut<T> for Vec<T> {
    fn prepend_mut(&mut self, s: &[T])
    where
        T: Clone,
    {
        let mut v = Vec::with_capacity(s.len() + self.len());
        v.extend_from_slice(s);
        v.extend_from_slice(self);
        *self = v;
    }
}

impl<T> VecPrependMut<T> for VecDeque<T> {
    fn prepend_mut(&mut self, s: &[T])
    where
        T: Clone,
    {
        let mut v = VecDeque::with_capacity(s.len() + self.len());
        v.extend(s.iter().cloned());
        v.extend(self.iter().cloned());
        *self = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend() {
        let v = vec![1, 2, 3];
        let new = v.prepend(&[4, 5, 6]);
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(new, vec![4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_prepend_mut() {
        let mut v = vec![1, 2, 3];
        v.prepend_mut(&[4, 5, 6]);
        assert_eq!(v, vec![4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_deque_prepend() {
        let v = VecDeque::from(vec![1, 2, 3]);
        let new = v.prepend(&[4, 5, 6]);
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(new, vec![4, 5, 6, 1, 2, 3]);
    }

    #[test]
    fn test_deque_prepend_mut() {
        let mut v = VecDeque::from(vec![1, 2, 3]);
        v.prepend_mut(&[4, 5, 6]);
        assert_eq!(v, vec![4, 5, 6, 1, 2, 3]);
    }
}
