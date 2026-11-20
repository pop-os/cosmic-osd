invalid-password = Nepareiza parole. Lūdzu, mēģiniet vēlreiz.
authentication-required = Nepieciešama autentifikācija
cancel = Atcelt
authenticate = Autentificēties
log-out = Atteikties
suspend = Iesnaudināt
restart = Pārstartēt
enter-bios = Atvērt BIOS
sound-settings = Skaņas iestatījumi
shutdown = Izslēgt
headphones = Austiņas
headset = Austiņas ar mikrofonu
confirm = Apstiprināt
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Izslēgt
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } tagad?
        [suspend] { suspend } tagad?
        [shutdown] { shutdown } tagad?
        [enter-bios] { enter-bios } tagad?
        [log-out] Aizvērt visas lietotnes un atteikties tagad?
        [confirm-device-type] Apstipriniet ierīces tipu
       *[other] Veikt atlasīto darbību tagad?
    }
confirm-body =
    Sistēma { $action ->
        [restart] tiks pārstartēta
        [suspend] tiks iesnaudināta
        [shutdown] tiks izslēgta
        [lock-screen] bloķēs ekrānu
        [log-out] atteiksies
        [enter-bios] tiks pārstartēta BIOS
       *[other] veiks atlasīto darbību
    } automātiski pēc { $countdown } s.
