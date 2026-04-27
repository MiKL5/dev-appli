#[derive(Debug)]
pub enum Operateur {
    Addition, Soustraction, Multiplication, Division,
    DivisionEuclidienne, DivisionEntiere, Puissance,
    RacineCarree, LogarithmeNeperien, Cosinus, Sinus, ValeurAbsolue,
}

impl Operateur {
    /// Détermine si l'opérateur ne requiert qu'un unique opérande (ex: sqrt).
    pub fn est_unaire(&self) -> bool {
        match self {
            Operateur::RacineCarree |
            Operateur::LogarithmeNeperien |
            Operateur::Cosinus |
            Operateur::Sinus |
            Operateur::ValeurAbsolue => true,
            _ => false,
        }
    }
}