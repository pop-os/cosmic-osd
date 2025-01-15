invalid-password = Mot de passe invalide. Veuillez réessayer.
authentication-required = Authentification nécessaire
cancel = Annuler
authenticate = S'authentifier
log-out = Se déconnecter
restart = Redémarrer
shutdown = Éteindre
confirm = Confirmer
cancel = Annuler
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] Mettre en veille
        [shutdown] {shutdown }
        [log-out] { log-out }
        *[other] { confirm }
}
confirm-title =
    { $action ->
        [restart] { restart }
        [suspend] Mettre le système en veille
        [shutdown] { shutdown }
        [log-out] Quitter toutes les applications et se déconnecter
        *[other] Appliquer l'option choisie
    } maintenant ?
confirm-body =
    Vous allez { $action ->
        [restart] redémarrer l'ordinateur
        [suspend] mettre l'ordinateur en veille
        [shutdown] éteindre l'ordinateur
        [lock-screen] verrouiller la session
        [log-out] déconnecter l'utilisateur
        *[other] appliquer l'option choisie
    } automatiquement dans { $countdown } secondes.

