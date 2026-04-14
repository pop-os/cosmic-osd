confirm = Vahvista
invalid-password = Väärä salasana. Yritä uudelleen.
cancel = Peru
log-out = Kirjaudu ulos
suspend = Lepotila
restart = Käynnistä uudelleen
enter-bios = Mene BIOS:iin
sound-settings = Ääniasetukset
shutdown = Sammuta
headphones = Kuulokkeet
headset = Kuulokemikrofoni
authentication-required = Tunnistautuminen vaaditaan
authenticate = Tunnistaudu
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Sammuta
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-body =
    Järjestelmä { $action ->
        [restart] käynnistyy uudelleen
        [suspend] siirtyy lepotilaan
        [shutdown] sammuu
        [lock-screen] lukitsee näytön
        [log-out] kirjaa ulos
        [enter-bios] käynnistyy uudelleen BIOS:iin
       *[other] toteuttaa valitun toiminnon
    } automaattisesti { $countdown } sekunnin kuluttua.
confirm-title =
    { $action ->
        [restart] { restart } nyt?
        [suspend] { suspend } nyt?
        [shutdown] { shutdown } nyt?
        [enter-bios] { enter-bios } nyt?
        [log-out] Lopetetaanko kaikki sovellukset ja kirjaudutaan ulos?
        [confirm-device-type] Vahvista laitetyyppi
       *[other] Toteutaanko valittu toiminto nyt?
    }
