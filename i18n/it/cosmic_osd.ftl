invalid-password = Password non valida, riprova.
authentication-required = Autenticazione necessaria
cancel = Annulla
authenticate = Esegui autenticazione
log-out = Disconnetti
restart = Riavvia
shutdown = Spegni
confirm = Conferma
cancel = Annulla
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Spegni
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Chiudi tutte le applicazioni e termina la sessione
        *[other] Applica l'azione selezionata
    } now?
confirm-body = 
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [lock-screen] Blocco schermo in corso
        [log-out] Disconnessione in corso
        *[other] L'azione selezionata
    } verrà eseguita tra { $countdown } secondi.
