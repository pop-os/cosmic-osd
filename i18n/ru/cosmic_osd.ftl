invalid-password = Неверный пароль. Пожалуйста, попробуйте еще раз.
authentication-required = Необходима аутентификация
cancel = Отмена
authenticate = Аутентифицировать
log-out = Выход из системы
restart = Перезагрузка
shutdown = Выключение
confirm = Подтвердить
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Выключить
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] Перезагрузить компьютер
        [suspend] Перейти в спящий режим
        [shutdown] Выключить компьютер
        [enter-bios] Перейти в BIOS
        [log-out] Закрыть все приложения и выйти
        [confirm-device-type] Подтвердить тип устройства
       *[other] Выполнить выбранное действие
    } сейчас?
confirm-body =
    Система { $action ->
        [restart] будет перезагружена
        [suspend] перейдёт в спящий режим
        [shutdown] будет выключена
        [lock-screen] заблокирует экран
        [log-out] выполнит выход
        [enter-bios] перезагрузится в BIOS
       *[other] выполнит выбранное действие
    } автоматически через { $countdown } сек.
suspend = Спящий режим
enter-bios = Перейти в BIOS
sound-settings = Параметры звука
headphones = Наушники
headset = Гарнитура
