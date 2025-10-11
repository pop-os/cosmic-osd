invalid-password = Ógilt lykilorð. Reyndu aftur.
authentication-required = Auðkenningar krafist
cancel = Hætta við
authenticate = Auðkenna
log-out = Skrá út
suspend = Svæfa
restart = Endurræsa
enter-bios = Opna BIOS
sound-settings = Hljóðstillingar
shutdown = Slökkva
headphones = Heyrnartól
headset = Höfuðtól
confirm = Staðfesta
confirm-body =
    Tölvan mun nú { $action ->
        [restart] endurræsast
        [suspend] sofna
        [shutdown] slökkva á sér
        [lock-screen] læsa skjánum
        [log-out] skrá út
        [enter-bios] endurræsast og fara í BIOS
       *[other] virkja völdu aðgerðina
    } slökkva á sér sjálfkrafa eftir { $countdown }.
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Slökkva
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
