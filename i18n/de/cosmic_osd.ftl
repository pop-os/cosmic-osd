invalid-password = Ungültiges Passwort. Bitte versuche es erneut.
authentication-required = Authentifizierung erforderlich
cancel = Abbrechen
authenticate = Authentifizieren
log-out = Abmelden
suspend = Bereitschaft
restart = Neustart
enter-bios = BIOS aufrufen
shutdown = Herunterfahren
confirm = Bestätigen
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Ausschalten
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] { enter-bios }
        [log-out] Alle Anwendungen beenden und abmelden
       *[other] Die ausgewählte Aktion
    } jetzt anwenden?
confirm-body =
    Das System wird in { $countdown } Sekunden automatisch { $action ->
        [restart] neu gestartet
        [suspend] in Bereitschaft versetzt
        [shutdown] ausgeschaltet
        [lock-screen] den Bildschirm sperren
        [log-out] dich abmelden
        [enter-bios] neu gestartet und das BIOS aufgerufen
       *[other] die ausgewählte Aktion anwenden
    }.
