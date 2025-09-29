invalid-password = Neplatné heslo. Zkuste to znovu.
authentication-required = Vyžadováno ověření
cancel = Zrušit
authenticate = Ověřit
log-out = Odhlásit se
suspend = Uspat
restart = Restartovat
enter-bios = Vstoupit do BIOSu
sound-settings = Nastavení zvuku
shutdown = Vypnout
headphones = Sluchátka
headset = Headset
confirm = Potvrdit
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Vypnout
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } nyní?
        [suspend] { suspend } nyní?
        [shutdown] { shutdown } nyní?
        [enter-bios] { enter-bios } nyní?
        [log-out] Ukončit všechny aplikace a odhlásit se nyní?
        [confirm-device-type] Potvrdit typ zařízení
       *[other] Použít vybranou akci nyní?
    }
confirm-body =
    Systém { $action ->
        [restart] se restartuje
        [suspend] se uspí
        [shutdown] se vypne
        [lock-screen] se uzamkne
        [log-out] odhlásí uživatele
        [enter-bios] vstoupí do BIOSu
       *[other] vykoná vybranou akci
    } automaticky za { $countdown ->
        [one] { $countdown } sekundu.
        [few] { $countdown } sekundy.
        [many] { $countdown } sekund.
       *[other] { $countdown } sekund.
    }
