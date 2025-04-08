invalid-password = Parolă invalidă. Încearcă din nou.
authentication-required = Autentificare necesară
cancel = Anulează
authenticate = Autentifică-te
log-out = Deconectează-te
restart = Repornește
enter-bios = Intră în BIOS
shutdown = Oprește
confirm = Confirmă
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Oprește
        [log-out] { log-out }
        [enter-bios] { enter-bios }
        *[other] { confirm }
}
confirm-title = 
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] { enter-bios }
        [log-out] Închide toate aplicațiile și deconectează-te
        *[other] Aplică acțiunea selectată
    } acum?
confirm-body = 
    Sistemul va { $action ->
        [restart] fi repornit
        [suspend] fi suspendat
        [shutdown] fi oprit
        [lock-screen] bloca ecranul
        [log-out] fi deconectat
        [enter-bios] reporni în BIOS
        *[other] aplica acțiunea selectată
    } automat în { $countdown } secunde.

