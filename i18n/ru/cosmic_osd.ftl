invalid-password = Неверный пароль. Пожалуйста, попробуйте еще раз.
authentication-required = Необходима аутентификация
cancel = Отмена
authenticate = Аутентифицировать
log-out = Выход из системы
restart = Перезагрузка
shutdown = Выключение
confirm = Подтвердить
cancel = Отмена
confirm-button = {
    $action -> 
        [restart] Перезагрузить
        [suspend] { suspend }
        [shutdown] Выключить
        [log-out] Выйти
        *[other] { confirm }
}
confirm-title = 
    { $action -> 
        [restart] Перезагрузить
        [suspend] Перейти в спящий режим
        [shutdown] Выключить
        [log-out] Закрыть все приложения и выйти
        *[other] Выполнить выбранное действие
    } сейчас?
confirm-body = 
    Система { $action ->
        [restart] будет перезагружена
        [suspend] перейдёт в спящий режим
        [shutdown] будет выключена
        [lock-screen] заблокирует экран
        [log-out] выполнит выход
        *[other] выполнит выбранное действие
    } автоматически через { $countdown } сек.

