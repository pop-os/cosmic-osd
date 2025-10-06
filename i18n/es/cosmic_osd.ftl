invalid-password = Contraseña inválida. Por favor, vuelva a introducirla.
authentication-required = Autentificación necesaria
cancel = Cancelar
authenticate = Autenticar
log-out = Cerrar sesión
enter-bios = Ingresar en el BIOS
suspend = Suspender
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
        [log-out] ¿Cerrar todas las aplicaciones y cerrar la sesión ahora?
        [confirm-device-type] Confirmar tipo de dispositivo
       *[other] ¿Realizar la acción seleccionada ahora?
    }
confirm-body =
    { $action ->
        [restart] El ordenador se reiniciará
        [suspend] El ordenador se suspenderá
        [shutdown] El ordenador se apagará
        [lock-screen] La pantalla se bloqueará
        [log-out] La sesión se cerrará
        [enter-bios] Reiniciar y entrar en la BIOS
       *[other] La acción seleccionada se realizará
    } automáticamente en { $countdown } segundos.
sound-settings = Ajustes de sonido
