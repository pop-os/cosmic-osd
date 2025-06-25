invalid-password = Érvénytelen jelszó. Próbáld újra.
authentication-required = Hitelesítés szükséges
cancel = Mégse
authenticate = Hitelesítés
log-out = Kijelentkezés
suspend = Felfüggesztés
restart = Újraindítás
enter-bios = BIOS-ba lépés
shutdown = Leállítás
confirm = Megerősítés
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Leállítás
        [log-out] { log-out }
        [enter-bios] {enter-bios}
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] Belépsz a BIOS-ba
        [log-out] Minden alkalmazás bezárása és kijelentkezés
        *[other] Alkalmazzuk a kiválasztott műveletet
    } most?
confirm-body = 
    A rendszer { $action ->
        [restart] újra fog indulni
        [suspend] felfüggesztésre kerül
        [shutdown] leáll
        [lock-screen] le fogja zárni a képernyőt
        [log-out] ki fog jelentkezni
        [enter-bios] újraindul a BIOS-ba
        *[other] alkalmazni fogja a kiválasztott műveletet
    } automatikusan { $countdown } másodperc múlva.
