invalid-password = Helytelen jelszó. Kérlek próbáld újra.
authentication-required = Azonosítás szükséges
cancel = Mégse
authenticate = Azonosítás
log-out = Kijelentkezés
restart = Újraindítás
shutdown = Leállítás
confirm = Megerősít
cancel = Mégse
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Leállítás
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Összes alkalmazásból kilépés és kijelentkezés
        *[other] Alkalmazza a kiválasztott műveletet
    } most?
confirm-body = 
    A rendszer { $action ->
        [restart] újra fog indulni
        [suspend] fel fogja függeszteni magát
        [shutdown] le fog állni
        [lock-screen] le fogja zárni a képernyőt
        [log-out] ki fog jelentkezni
        *[other] alkalmazni fogja a kiválasztott műveletet
    } automatikusan { $countdown } másodpercen belül.

