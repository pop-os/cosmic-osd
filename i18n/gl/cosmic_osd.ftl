cancel = Cancelar
confirm = Confirmar
restart = Reiniciar
shutdown = Apagar
suspend = Suspender
invalid-password = Contrasinal incorrecto. Téntao de novo.
authentication-required = Autenticación necesaria
authenticate = Autenticar
log-out = Pechar Sesión
enter-bios = Entrar na BIOS
sound-settings = Configuración de Son
headphones = Auriculares
headset = Auriculares con micrófono
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Apagar
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
        [log-out] Pechar todas as aplicacións e pechar a sesión agora?
        [confirm-device-type] Confirmar o tipo de dispositivo
       *[other] Aplicar a acción seleccionada agora?
    }
confirm-body =
    O sistema { $action ->
        [restart] reiniciarase
        [suspend] suspenderase
        [shutdown] apagarase
        [lock-screen] bloqueará a pantalla
        [log-out] pechará a sesión
        [enter-bios] reiniciarase para entrar na BIOS
       *[other] aplicará a acción seleccionada
    } automaticamente en { $countdown } segundos.
