pub(crate) fn true_() -> bool {
    true
}

pub(crate) fn eq_default<T: Default + PartialEq>(t: &T) -> bool {
    t.eq(&Default::default())
}
