cancel = Afbryd
confirm = Bekræft
restart = Genstart
suspend = Slumre
shutdown = Luk ned
log-out = Log Ud
invalid-password = Ugyldig adgangskode. Prøv venligst igen.
authentication-required = Godkendelse Påkrævet
authenticate = Godkend
sound-settings = Lydindstillinger
headphones = Hovedtelefoner
headset = Hovedsæt
enter-bios = Gå til BIOS
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Sluk
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } nu?
        [suspend] { suspend } nu?
        [shutdown] { shutdown } nu?
        [enter-bios] { enter-bios } nu?
        [log-out] Forlad alle applikationer og log ud nu?
        [confirm-device-type] Bekræft Enhedstype
       *[other] Anvend den valgte handling nu?
    }
confirm-body =
    Systemet vil automatisk { $action ->
        [restart] genstart
        [suspend] slumre
        [shutdown] slukke
        [lock-screen] låse skærmen
        [log-out] logge ud
        [enter-bios] genstarte til BIOS
       *[other] anvende den valgte handling
    } om { $countdown } sekunder.
