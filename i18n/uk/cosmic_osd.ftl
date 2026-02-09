invalid-password = Неправильний пароль. Повторіть спробу.
authentication-required = Автентифікація необхідна
cancel = Скасувати
authenticate = Автентифікувати
enter-bios = Увійти в BIOS
sound-settings = Налаштування звуку
shutdown = Вимкнути
headphones = Навушники
log-out = Вийти
suspend = Призупинити
restart = Перезавантажити
headset = Гарнітура
confirm = Підтвердити
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Вимкнути
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } зараз?
        [suspend] { suspend } зараз?
        [shutdown] { shutdown } зараз?
        [enter-bios] { enter-bios } зараз?
        [log-out] Закрити застосунки й вийти зараз?
        [confirm-device-type] Підтвердити тип пристрою
       *[other] Виконати обрану дію зараз?
    }
confirm-body =
    Система { $action ->
        [restart] перезавантажиться
        [suspend] призупинеться
        [shutdown] вимкнеться
        [lock-screen] заблокує екран
        [log-out] завершить сеанс
        [enter-bios] перезавантажиться у BIOS
       *[other] виконає обрану дію
    } автоматично через { $countdown } секунд.
