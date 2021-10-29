use crate::Play;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observer<'a, T: Play> {
    pub turn_num: u64,
    pub settings: &'a <T as Play>::Settings,
    pub public_info: &'a <T as Play>::PublicInfo,
}

pub trait Observe<T: Play> {
    fn observe(&self) -> Observer<'_, T>;
}
