invalid-password = Nieprawidłowe hasło. Spróbuj ponownie.
authentication-required = Wymagane Uwierzytelnienie
cancel = Anuluj
authenticate = Uwierzytelnij
log-out = Wyloguj się
suspend = Wstrzymaj
restart = Uruchom Ponownie
enter-bios = Przejdź do BIOS
shutdown = Wyłącz
confirm = Potwierdź
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] Uruchomić ponownie
        [suspend] Wstrzymać?
        [shutdown] Wyłączyć?
        [enter-bios] Przejść do BIOS?
        [log-out] Zamknąć wszystkie aplikacje i się wylogować?
        [confirm-device-type] Potwierdź typ urządzenia
       *[other] Wykonać wybraną akcję?
    }
confirm-body =
    Urządzenie automatycznie { $action ->
        [restart] uruchomi się ponownie
        [suspend] przejdzie w stan uśpienia
        [shutdown] wyłączy się
        [lock-screen] zablokuje ekran
        [log-out] wyloguje się
        [enter-bios] przejdzie do BIOS
       *[other] wykona wybraną akcję
    } za { $countdown ->
        [1] sekundę.
        [few] { $countdown } sekundy.
       *[other] { $countdown } sekund.
    }
sound-settings = Ustawienia dźwięku
headphones = Słuchawka
headset = Słuchawki
