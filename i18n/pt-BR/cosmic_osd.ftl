invalid-password = Senha inválida. Tente novamente.
authentication-required = Autenticação necessária
cancel = Cancelar
authenticate = Autenticar
log-out = Sair
restart = Reiniciar
enter-bios = Entrar na BIOS
shutdown = Desligar
confirm = Confirmar
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Desligar
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
        [log-out] Fechar todos os aplicativos e sair
        *[other] Aplicar a ação selecionada
    } agora?
confirm-body = 
    O sistema irá { $action ->
        [restart] reiniciar
        [suspend] suspender
        [shutdown] desligar
        [lock-screen] bloquear a tela
        [log-out] sair
        [enter-bios] reiniciar e entrar na BIOS
        *[other] aplicar a ação selecionada
    } automaticamente em { $countdown } segundos.

