invalid-password = Neplatné heslo. Skúste to znova.
authentication-required = Vyžaduje sa overenie
cancel = Zrušiť
authenticate = Overiť
log-out = Odhlásiť sa
suspend = Uspat
restart = Reštartovať
enter-bios = Vstúpiť do BIOSu
sound-settings = Nastavenia zvuku
shutdown = Vypnúť
headphones = Slúchadlá
headset = Headset
confirm = Potvrdiť
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Vypnúť
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] Vstúpiť do BIOSu?
        [log-out] Ukončiť všetky aplikácie a odhlásiť sa?
        [confirm-device-type] Potvrdiť typ zariadenia
        *[other] Použiť vybranú akciu?
    }
confirm-body =
    Systém sa { $action ->
        [restart] reštartuje
        [suspend] uspí
        [shutdown] vypne
        [lock-screen] uzamkne
        [log-out] odhlási
        [enter-bios] reštartuje do BIOSu
        *[other] vykoná vybranú akciu
    } automaticky o { $countdown ->
        [one] { $countdown } sekundu.
        [few] { $countdown } sekundy.
        [many] { $countdown } sekúnd.
        *[other] { $countdown } sekundy.
    }
