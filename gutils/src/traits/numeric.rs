macro_rules! implementer {
    ($trait:path, $($t:ty),+ $(,)?) => {
        $(impl $trait for $t {})+
    };
}

#[allow(unused)]
pub trait IntegerMoreThanEightBytes {}

#[cfg(target_pointer_width = "32")]
implementer!(IntegerMoreThanEightBytes, u64, i64);

#[cfg(not(target_pointer_width = "32"))]
implementer!(IntegerMoreThanEightBytes, u64, i64, usize, isize);

#[allow(unused)]
pub trait IntegerMoreThanFourBytes {}

impl<T: IntegerMoreThanEightBytes> IntegerMoreThanFourBytes for T {}

#[cfg(target_pointer_width = "32")]
implementer!(IntegerMoreThanFourBytes, u32, i32, usize, isize,);

#[cfg(not(target_pointer_width = "32"))]
implementer!(IntegerMoreThanFourBytes, u32, i32);

#[allow(unused)]
pub trait IntegerMoreThanTwoBytes {}

impl<T: IntegerMoreThanFourBytes> IntegerMoreThanTwoBytes for T {}

implementer!(IntegerMoreThanTwoBytes, u16, i16);

#[allow(unused)]
pub trait Integer {}

impl<T: IntegerMoreThanTwoBytes> Integer for T {}

implementer!(Integer, u8, i8);
