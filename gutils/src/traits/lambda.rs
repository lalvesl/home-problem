//Someday this will become a trait, which will be implemented in everything possible
#[allow(dead_code)]
pub fn lbd<I, F, O>(element: I, mut predicate: F) -> O
where
    I: Sized,
    F: FnMut(I) -> O,
    O: Sized,
{
    predicate(element)
}
