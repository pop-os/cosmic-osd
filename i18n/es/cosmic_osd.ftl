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
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] { log-out }
        [enter-bios] {enter-bios}
        *[other] { confirm }
}
confirm-title =
    ¿{ $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] {enter-bios}
        [log-out] ¿Cerrar todas las aplicaciones y cerrar la sesión
        *[other] ¿Realizar la acción seleccionada
    }?
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
