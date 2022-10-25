pub use internment::Intern as Id;
pub type IString = Id<str>;

#[inline]
pub fn intern(str: &str) -> IString {
    IString::from(str)
}
