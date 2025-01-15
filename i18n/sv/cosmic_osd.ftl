invalid-password = Ogiltigt lösenord. Var god försök igen.
authentication-required = Autentisering krävs
cancel = Avbryt
authenticate = Autentisera
log-out = Вийти
restart = Перезавантажити
shutdown = Вимкнути
confirm = Підтвердити
cancel = Скасувати
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Вимкнути
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Закрити всі застосунки та вийти
        *[other] Виконати вибрану дію
    } зараз?
confirm-body = 
    Система { $action ->
        [restart] перезавантажиться
        [suspend] призупиниться
        [shutdown] вимкнеться
        [lock-screen] заблокує екран
        [log-out] виконає вихід
        *[other] Виконати вибрану дію
    } автоматично за { $countdown } { $countdown ->
        [one] секунду
        [few] секунди
        *[other] секунд
    }.
