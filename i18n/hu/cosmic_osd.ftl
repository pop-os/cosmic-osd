invalid-password = Érvénytelen jelszó. Próbáld újra.
authentication-required = Hitelesítés szükséges
cancel = Mégse
authenticate = Hitelesítés
log-out = Kijelentkezés
suspend = Felfüggesztés
restart = Újraindítás
enter-bios = BIOS-ba lépés
sound-settings = Hangbeállítások
shutdown = Leállítás
headphones = Fejhallgató
headset = Headset
confirm = Megerősítés
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Leállítás
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } most?
        [suspend] { suspend } most?
        [shutdown] { shutdown } most?
        [enter-bios] { enter-bios } most?
        [log-out] Minden alkalmazás bezárása és kijelentkezés most?
        [confirm-device-type] Eszköztípus megerősítése
       *[other] A kiválasztott művelet alkalmazása most?
    }
confirm-body =
    A rendszer automatikusan { $action ->
        [restart] újra fog indulni
        [suspend] felfüggesztésre kerül
        [shutdown] leáll
        [lock-screen] le fogja zárni a képernyőt
        [log-out] ki fog jelentkezni
        [enter-bios] újraindul a BIOS-ba
       *[other] alkalmazni fogja a kiválasztott műveletet
    } { $countdown } másodperc múlva.
