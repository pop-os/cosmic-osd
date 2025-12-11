invalid-password = Senha inválida. Tente novamente.
authentication-required = Autenticação necessária
cancel = Cancelar
authenticate = Autenticar
log-out = Sair
suspend = Suspender
restart = Reiniciar
enter-bios = Entrar no BIOS
shutdown = Desligar
confirm = Confirmar
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Desligar
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } agora?
        [suspend] { suspend } agora?
        [shutdown] { shutdown } agora?
        [enter-bios] { enter-bios } agora?
        [log-out] Fechar todos os aplicativos e sair agora?
        [confirm-device-type] Confirmar tipo de dispositivo
       *[other] Aplicar a ação selecionada agora?
    }
confirm-body =
    O sistema irá { $action ->
        [restart] reiniciar
        [suspend] suspender
        [shutdown] desligar
        [lock-screen] bloquear a tela
        [log-out] sair
        [enter-bios] reiniciar e entrar no BIOS
       *[other] aplicar a ação selecionada
    } automaticamente em { $countdown } segundos.
sound-settings = Configurações de som
headphones = Fones de ouvido
headset = Headset
