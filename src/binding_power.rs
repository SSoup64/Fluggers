#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum BindingPower {
    Min,
    Additive,
    Multiplicative,
    Group,
}
