invalid-password = Érvénytelen jelszó. Kérlek próbáld újra.
authentication-required = Hitelesítés szükséges
cancel = Mégse
authenticate = Hitelesítés
log-out = Kijelentkezés
restart = Újraindítás
enter-bios = BIOS-ba lépés
shutdown = Leállítás
confirm = Megerősítés
confirm-button = {
    $action -> 
        [restart] Újraindítás
        [suspend] Felfüggesztés
        [shutdown] Leállítás
        [log-out] Kijelentkezés
        [enter-bios] BIOS-ba lépés
        *[other] Megerősítés
}
confirm-title = 
    { $action -> 
        [restart] Újraindítás
        [suspend] Felfüggesztés
        [shutdown] Leállítás
        [enter-bios] Belép a BIOS-ba
        [log-out] Minden alkalmazás bezárása és kijelentkezés
        *[other] Alkalmazza a kiválasztott műveletet
    } most?
confirm-body = 
    A rendszer { $action ->
        [restart] újra fog indulni
        [suspend] felfüggesztésre kerül
        [shutdown] le fog állni
        [lock-screen] le fogja zárni a képernyőt
        [log-out] ki fog jelentkezni
        [enter-bios] újraindul a BIOS-ba
        *[other] alkalmazni fogja a kiválasztott műveletet
    } automatikusan { $countdown } másodpercen belül.
