use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone)]
pub enum IncResult<T: Incrementable> {
    Ok(T),
    OutOfBounds,
    UnknownError(String),
}

impl<T: Incrementable> IncResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            IncResult::Ok(x) => x,
            _ => panic!("Tried unwrapping non-Ok IncResult value")
        }
    }
}

pub struct IncrementFactory<T: Incrementable> {
    _t: PhantomData<T>,
}

impl<T: Incrementable> IncrementFactory<T> {
    pub fn new() -> Self {
        IncrementFactory {
            _t: PhantomData
        }
    }

    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, IncrementFactory::<i32>::new().produce(1).result().unwrap());
    /// ```
    #[inline]
    pub fn produce(&self, value: T) -> Increment<T> {
        Increment::<T>::new(value)
    }
}

pub struct Increment<T: Incrementable> {
    incrementable: T,
    result: Option<IncResult<T>>,
}

impl<T: Incrementable> Increment<T> {
    pub fn new(incrementable: T) -> Self {
        Increment {
            incrementable,
            result: None,
        }
    }
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, Increment::<i8>::new(1).result().unwrap());
    /// ```
    pub fn result(&mut self) -> IncResult<T> {
        match &self.result {
            Some(res) => res.clone(),
            None => {
                self.execute();
                self.result.clone().unwrap()
            }
        }
    }

    fn execute(&mut self) {
        self.result = Some(self.incrementable.increment());
    }
}

pub trait Incrementable: Sized + Clone {
    fn increment(&self) -> IncResult<Self>;
}

impl Incrementable for i8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1i8.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, i8::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &i8::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for i16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1i16.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, i16::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &i16::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for i32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1i32.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, i32::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &i32::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for i64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1i64.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, i64::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &i64::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for i128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1i128.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, i128::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &i128::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for u8 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1u8.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, u8::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &u8::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for u16 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1u16.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, u16::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &u16::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for u32 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1u32.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, u32::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &u32::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for u64 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1u64.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, u64::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &u64::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for u128 {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(2, 1u128.increment().unwrap());
    /// assert_eq!(IncResult::OutOfBounds, u128::max_value().increment());
    /// ```
    #[inline]
    fn increment(&self) -> IncResult<Self> {
        if self == &u128::max_value() {
            IncResult::OutOfBounds
        } else {
            IncResult::Ok(self + 1)
        }
    }
}

impl Incrementable for String {
    /// # Examples:
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!("2", "1".to_string().increment().unwrap());
    /// ```
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!("-2", "-3".to_string().increment().unwrap());
    /// ```
    ///
    /// ```
    /// use increment::*;
    ///
    /// assert_eq!(IncResult::OutOfBounds, u128::max_value().to_string().increment());
    /// ```
    fn increment(&self) -> IncResult<Self> {
        let trimmed = self.trim();
        if trimmed.starts_with("-") {
            match self.parse::<i128>() {
                Ok(i) => match i.increment() {
                    IncResult::Ok(i) => IncResult::Ok(i.to_string()),
                    IncResult::OutOfBounds => IncResult::OutOfBounds,
                    IncResult::UnknownError(e) => IncResult::UnknownError(e),
                },
                Err(e) => IncResult::UnknownError(e.to_string()),
            }
        } else {
            match self.parse::<u128>() {
                Ok(i) => match i.increment() {
                    IncResult::Ok(i) => IncResult::Ok(i.to_string()),
                    IncResult::OutOfBounds => IncResult::OutOfBounds,
                    IncResult::UnknownError(e) => IncResult::UnknownError(e),
                },
                Err(e) => IncResult::UnknownError(e.to_string()),
            }
        }
    }
}