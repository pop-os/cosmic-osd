invalid-password = Неправильний пароль. Будь ласка, повторіть спробу.
authentication-required = Потрібна автентифікація
cancel = Скасувати
authenticate = Автентифікація
enter-bios = Увійти у налаштування BIOS
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
        [log-out] Вийти з усіх застосунків та завершити сеанс зараз?
        [confirm-device-type] Підтвердіть тип пристрою
       *[other] Виконати обрану дію зараз?
    }
confirm-body =
    Система { $action ->
        [restart] перезавантажиться
        [suspend] призупинеться
        [shutdown] вимкнеться
        [lock-screen] заблокує екран
        [log-out] завершить сеанс
        [enter-bios] перезавантажиться у налаштування BIOS
       *[other] виконає обрану дію
    } автоматично за { $countdown } { $countdown ->
        [one] секунду
        [few] секунди
       *[other] секунд
    }.
