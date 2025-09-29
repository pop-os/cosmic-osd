invalid-password = Palavra-passe inválida. Tente novamente.
authentication-required = Autenticação necessária
cancel = Cancelar
authenticate = Autenticar
log-out = Terminar sessão
restart = Reiniciar
shutdown = Encerrar
confirm = Confirmar
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Encerrar
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Sair de todas as aplicações e terminar sessão
        *[other] Aplicar a ação selecionada
    } agora?
confirm-body = 
    O sistema vai { $action ->
        [restart] reiniciar
        [suspend] suspender
        [shutdown] encerrar
        [lock-screen] bloquear o ecrã
        [log-out] terminar sessão
        *[other] aplicar a ação selecionada
    } automatica em { $countdown } segundos.
