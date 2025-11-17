invalid-password = Mot de passe invalide. Veuillez réessayer.
authentication-required = Authentification nécessaire
cancel = Annuler
authenticate = S'authentifier
log-out = Se déconnecter
restart = Redémarrer
suspend = Veille
enter-bios = Entrer dans le BIOS
shutdown = Éteindre
confirm = Confirmer
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Éteindre
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } maintenant ?
        [suspend] { suspend } maintenant ?
        [shutdown] { shutdown } maintenant ?
        [enter-bios] { enter-bios } maintenant ?
        [log-out] Quitter toutes les applications et se déconnecter maintenant ?
        [confirm-device-type] Confirmer le type d'appareil
       *[other] Appliquer l'option choisie maintenant ?
    }
confirm-body =
    Cet ordinateur { $action ->
        [restart] redémarrera
        [suspend] se mettra en veille
        [shutdown] s'éteindra
        [lock-screen] se verrouillera
        [log-out] se déconnectera
        [enter-bios] redémarrera et entrera dans le BIOS
       *[other] appliquera l'option choisie
    } automatiquement dans { $countdown } secondes.
sound-settings = Paramètres audio
headphones = Ecouteurs
headset = Casque
