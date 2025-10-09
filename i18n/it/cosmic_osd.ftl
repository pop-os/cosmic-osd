invalid-password = Password non valida, per favore riprova.
authentication-required = Autenticazione necessaria
cancel = Annulla
authenticate = Autentica
log-out = Disconnetti
suspend = Sospendi
restart = Riavvia
enter-bios = Riavvia nel BIOS
sound-settings = Impostazioni audio
shutdown = Spegni
headphones = Cuffie
headset = Auricolari con microfono
confirm = Conferma
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Spegni
        [log-out] { log-out }
        [enter-bios] {enter-bios}
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] {enter-bios}
        [log-out] Chiudere tutte le applicazioni e disconnettere
        *[other] Applicare l'azione selezionata
    } adesso?
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
