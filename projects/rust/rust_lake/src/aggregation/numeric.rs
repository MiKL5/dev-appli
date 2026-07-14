use std::ops::Add;


/// Définir un contrat commun à tout type numérique pouvant être agrégé.
///
/// Illustrer les types génériques bornés par trait (trait bounds) : plutôt que
/// de dupliquer la structure Aggregator pour u64, f64 ou u32, une seule
/// implémentation générique couvre tous les types respectant ce contrat.
pub trait Numeric:
    Copy + Add<Output = Self> + PartialOrd + Default + std::fmt::Display
{
}

impl Numeric for u64 {}
impl Numeric for f64 {}
impl Numeric for u32 {}