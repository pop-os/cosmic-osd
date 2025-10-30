invalid-password = Password non valida, per favore riprova.
authentication-required = Autenticazione necessaria
cancel = Annulla
authenticate = Autentica
log-out = Disconnetti
restart = Riavvia
enter-bios = Riavvia nel BIOS
shutdown = Spegni
confirm = Conferma
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Spegni
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } ora?
        [suspend] { suspend } ora?
        [shutdown] { shutdown } ora?
        [enter-bios] { enter-bios } ora?
        [log-out] Chiudere tutte le applicazioni e disconnettere ora?
        [confirm-device-type] Conferma tipo di dispositivo
       *[other] Applicare l'azione selezionata ora?
    }
confirm-body =
    Il sistema { $action ->
        [restart] verrà riavviato
        [suspend] verrà sospeso
        [shutdown] verrà spento
        [lock-screen] verrà bloccato
        [log-out] verrà disconnesso
        [enter-bios] verrà riavviato nel BIOS
       *[other] applicherà l'azione selezionata
    } automaticamente in { $countdown } secondi.
suspend = Sospendi
sound-settings = Impostazioni audio
headphones = Cuffie
headset = Cuffie con microfono
