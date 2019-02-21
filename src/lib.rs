pub trait Incrementable {
    fn increment(&self) -> Option<Box<Self>>;
}

impl Incrementable for i8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<i8>::new(2)), 1.increment());
    /// assert_eq!(None, i8::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &i8::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for i16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<i16>::new(2)), 1.increment());
    /// assert_eq!(None, i16::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &i16::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for i32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<i32>::new(2)), 1.increment());
    /// assert_eq!(None, i32::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &i32::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for i64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<i64>::new(2)), 1.increment());
    /// assert_eq!(None, i64::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &i64::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for i128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<i128>::new(2)), 1.increment());
    /// assert_eq!(None, i128::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &i128::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for u8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<u8>::new(2)), 1.increment());
    /// assert_eq!(None, u8::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &u8::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for u16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<u16>::new(2)), 1.increment());
    /// assert_eq!(None, u16::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &u16::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for u32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<u32>::new(2)), 1.increment());
    /// assert_eq!(None, u32::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &u32::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for u64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<u64>::new(2)), 1.increment());
    /// assert_eq!(None, u64::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &u64::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}

impl Incrementable for u128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::Incrementable;
    ///
    /// assert_eq!(Some(Box::<u128>::new(2)), 1.increment());
    /// assert_eq!(None, u128::max_value().increment());
    /// ```
    fn increment(&self) -> Option<Box<Self>> {
        if self == &u128::max_value() {
            None
        } else {
            Some(Box::new(self + 1))
        }
    }
}