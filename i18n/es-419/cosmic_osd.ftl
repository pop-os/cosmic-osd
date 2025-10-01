invalid-password = Contraseña incorrecta. Por favor, inténtalo de nuevo.
authentication-required = Autenticación requerida
cancel = Cancelar
authenticate = Autenticar
log-out = Cerrar sesión
restart = Reiniciar
shutdown = Apagar
confirm = Confirmar
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
        [restart] ¿{ restart } ahora?
        [suspend] ¿{ suspend } ahora?
        [shutdown] ¿{ shutdown } ahora?
        [enter-bios] ¿{ enter-bios } ahora?
        [log-out] ¿Cerrar todas las aplicaciones y cerrar sesión ahora?
        [confirm-device-type] Confirmar tipo de dispositivo
       *[other] ¿Realizar la operación seleccionada ahora?
    }
confirm-body =
    El sistema { $action ->
        [restart] se reiniciará
        [suspend] se suspenderá
        [shutdown] se apagará
        [lock-screen] bloqueará la pantalla
        [log-out] cerrará sesión
        [enter-bios] se reiniciará y entrará al BIOS
       *[other] realizará la operación seleccionada
    } automáticamente en { $countdown } segundos.
suspend = Suspender
enter-bios = Entrar al BIOS
sound-settings = Configuración de sonido
headphones = Audífonos
headset = Audífonos con micrófono
