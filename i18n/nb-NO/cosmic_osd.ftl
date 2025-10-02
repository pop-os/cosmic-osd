cancel = Avbryt
restart = Start på nytt
shutdown = Slå av
headphones = Hodetelefoner
headset = Headset
confirm = Bekreft
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Slå av
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } nå?
        [suspend] { suspend } nå?
        [shutdown] { shutdown } nå?
        [enter-bios] { enter-bios } nå?
        [log-out] Avslutt alle programmer og logg ut nå?
        [confirm-device-type] Bekreft Enhetstype
       *[other] Bruk den valgte handlingen nå?
    }
confirm-body =
    Systemet vil { $action ->
        [restart] starte på nytt
        [suspend] gå i hvilemodus
        [shutdown] slås av
        [lock-screen] låse skjermen
        [log-out] logge ut
        [enter-bios] starte på nytt i BIOS
       *[other] bruke den valgte handlingen
    } automatisk om { $countdown } sekunder.
invalid-password = Feil passord, Prøv igjen.
authentication-required = Autentisering Kreves
authenticate = Autentiser
log-out = Logg ut
suspend = Dvalemodus
enter-bios = Gå inn i BIOS
sound-settings = Lydinnstillinger
