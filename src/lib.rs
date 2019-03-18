pub trait Incrementable: Sized {
    fn increment(&self) -> Option<Self>;
}

impl Incrementable for i8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, i8::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &i8::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for i16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, i16::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &i16::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for i32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, i32::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &i32::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for i64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, i64::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &i64::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for i128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, i128::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &i128::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for u8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, u8::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &u8::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for u16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, u16::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &u16::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for u32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, u32::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &u32::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for u64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, u64::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &u64::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}

impl Incrementable for u128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(2), 1.increment());
    /// assert_eq!(None, u128::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Self> {
        if self == &u128::max_value() {
            None
        } else {
            Some(self + 1)
        }
    }
}