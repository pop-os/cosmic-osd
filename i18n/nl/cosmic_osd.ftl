invalid-password = Invalid password. Please try again.
authentication-required = Authenticatie vereist
cancel = Annuleren
authenticate = Authenticeren
log-out = Afmelden
suspend = Slaapstand
restart = Opnieuw opstrarten
enter-bios = Ga naar BIOS
shutdown = Afsluiten
confirm = Bevestigen
confirm-button = {
    $action -> 
        [restart] Opnieuw opstarten
        [suspend] Slaapstand
        [shutdown] Afsluiten
        [log-out] Afmelden
        [enter-bios] Ga naar BIOS
        *[other] Bevestigen
}
confirm-title = 
    Nu { $action -> 
        [restart] opnieuw opstarten?
        [suspend] in slaapstand gaan?
        [shutdown] afsluiten?
        [enter-bios] de BIOS openen?
        [log-out] alle applicaties sluiten en afmelden?
        *[other] de geselecteerde actie uitvoeren?
    } 
confirm-body = 
    { $action ->
        [restart] De computer start na { $countdown } seconden automatisch opnieuw op.
        [suspend] De computer gaat na { $countdown } seconden automatisch in slaapstand.
        [shutdown] De computer wordt na { $countdown } seconden automatisch afgesloten.
        [lock-screen] De schermvergrendeling wordt na { $countdown } seconden automatisch actief.
        [log-out] De gebruiker wordt na { $countdown } seconden automatisch afgemeld.
        [enter-bios] De computer start na { $countdown } seconden automatisch opnieuw op in de BIOS.
        *[other] De geselecteerde actie wordt na { $countdown } seconden automatisch uitgevoerd.
    } 
