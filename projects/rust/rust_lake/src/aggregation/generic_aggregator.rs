use crate::aggregation::numeric::Numeric;
use crate::domain::RustLakeError;


/// Représenter un agrégateur générique capable de traiter tout type numérique
/// respectant le contrat Numeric.
///
/// Cette structure démontre l'usage de types génériques (T: Numeric) associés
/// à des opérateurs surchargés (Add) pour accumuler des statistiques sans
/// jamais connaître à l'avance le type concret manipulé.
#[derive(Debug, Clone)]
pub struct Aggregator<T: Numeric> {
    pub total: T,
    pub count: usize,
    pub max: T,
}

impl<T: Numeric> Aggregator<T> {
    /// Instancier un agrégateur vide avec des valeurs par défaut.
    pub fn new() -> Self {
        Aggregator {
            total: T::default(),
            count: 0,
            max: T::default(),
        }
    }

    /// Ajouter une observation et mettre à jour les statistiques cumulées.
    ///
    /// Utiliser un contrôle de flux conditionnel simple pour maintenir le maximum
    /// sans dépendre d'une bibliothèque tierce.
    pub fn add(&mut self, value: T) {
        self.total = self.total + value;
        self.count += 1;
        if value > self.max {
            self.max = value;
        }
    }

    /// Fusionner un autre agrégateur dans l'agrégateur courant.
    ///
    /// Retourner une erreur explicite plutôt qu'un panic en cas d'état incohérent
    /// permet de préserver la robustesse lors de la fusion de lots parallèles.
    pub fn merge(&mut self, other: &Aggregator<T>) -> Result<(), RustLakeError> {
        if other.count == 0 {
            return Ok(());
        }
        self.total = self.total + other.total;
        self.count += other.count;
        if other.max > self.max {
            self.max = other.max;
        }
        Ok(())
    }
}

impl<T: Numeric> Default for Aggregator<T> {
    /// Fournir une implémentation par défaut cohérente avec la méthode new().
    fn default() -> Self {
        Self::new()
    }
}