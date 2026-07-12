use crate::erreurs::VaultError;

/// Vérifier l'habilitation via une combinaison explicite d'opérateurs logiques.
pub fn verifier_habilitation(niveau_utilisateur: u8, niveau_requis: u8, est_admin: bool) -> Result<(), VaultError> {
    let autorise = est_admin || (niveau_utilisateur >= niveau_requis && niveau_utilisateur > 0);
    if !autorise {
        return Err(VaultError::AccesRefuse(niveau_requis));
    }
    Ok(())
}