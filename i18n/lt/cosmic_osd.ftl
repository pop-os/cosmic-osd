invalid-password = Neteisingas slaptažodis. Bandykite dar kartą.
cancel = Atšaukti
authentication-required = Reikalinga Autentifikacija
authenticate = Autentifikuoti
log-out = Atsijungti
confirm = Patvirtinti
restart = Paleisti iš naujo
enter-bios = Įeiti į BIOS
sound-settings = Garso nustatymai
headphones = Ausinės
headset = Mikrofonas su ausinėmis
confirm-title =
    { $action ->
        [restart] { restart } dabar?
        [suspend] { suspend } dabar?
        [shutdown] { shutdown } dabar?
        [enter-bios] { enter-bios } dabar?
        [log-out] Išjungti visas aplikacijas ir atsijungti dabar?
        [confirm-device-type] Patvirtinkite įrenginio tipą
       *[other] Taikyti pasirinktą veiksmą dabar?
    }
confirm-body =
    Sistema { $action ->
        [restart] paleis iš naujo įrenginį
        [suspend] užmigdys įrenginį
        [shutdown] išjungs įrenginį
        [lock-screen] užrakins ekraną
        [log-out] atjungs dabartinį naudotoją
        [enter-bios] įjungs iš naujo įrenginį ir paleis BIOS
       *[other] taikys pasirinktą veiksmą
    } automatiškai po { $countdown } sekundžių.
