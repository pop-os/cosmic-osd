invalid-password = Palavra-passe inválida. Tente novamente.
authentication-required = Autenticação necessária
cancel = Cancelar
authenticate = Autenticar
log-out = Terminar sessão
restart = Reiniciar
shutdown = Encerrar
confirm = Confirmar
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Encerrar
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
        [log-out] Sair de todas as aplicações e terminar sessão agora?
        [confirm-device-type] Confirmar tipo de dispositivo
       *[other] Aplicar a ação selecionada agora?
    }
confirm-body =
    O sistema vai { $action ->
        [restart] reiniciar
        [suspend] suspender
        [shutdown] encerrar
        [lock-screen] bloquear o ecrã
        [log-out] terminar sessão
        [enter-bios] reiniciar e entrar no BIOS
       *[other] aplicar a ação selecionada
    } automaticamente dentro de { $countdown } segundos.
suspend = Suspender
enter-bios = Entrar no BIOS
sound-settings = Definições de som
headphones = Auscultadores
headset = Auscultadores com Microfone
