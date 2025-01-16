invalid-password = Contraseña incorrecta. Por favor, inténtalo de nuevo.
authentication-required = Autenticación requerida
cancel = Cancelar
authenticate = Autenticar
log-out = Cerrar sesión
restart = Reiniciar
shutdown = Apagar
confirm = Confirmar
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Apagar
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    ¿{ $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Cerrar todas las aplicaciones y cerrar sesión
        *[other] Realizar la sesión seleccionada
    } ahora?
confirm-body = 
    El sistema { $action ->
        [restart] se reiniciará
        [suspend] se suspenderá
        [shutdown] se apagará
        [lock-screen] bloqueará la pantalla
        [log-out] cerrará sesión
        *[other] realizará la acción seleccionada
    } automáticamente en { $countdown } segundos.
