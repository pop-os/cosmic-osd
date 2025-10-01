invalid-password = Felaktigt lösenord. Försök igen.
authentication-required = Autentisering krävs
cancel = Avbryt
authenticate = Autentisera
log-out = Logga ut
suspend = Vänteläge
restart = Starta om
enter-bios = Gå in i BIOS
sound-settings = Ljudinställningar
shutdown = Stäng av
headphones = Hörlurar
headset = Headset
confirm = Bekräfta
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Stäng av
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
        [log-out] Avsluta alla program och logga ut nu?
        [confirm-device-type] Bekräfta enhetstyp
       *[other] Tillämpa vald åtgärd nu?
    }
confirm-body =
    Systemet kommer att { $action ->
        [restart] starta om
        [suspend] försättas i viloläge
        [shutdown] stängas av
        [lock-screen] låsa skärmen
        [log-out] logga ut
        [enter-bios] starta om till BIOS
       *[other] tillämpa vald åtgärd
    } automatiskt om { $countdown } sekunder.
