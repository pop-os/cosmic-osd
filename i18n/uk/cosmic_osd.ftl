invalid-password = Неправильний пароль. Повторіть спробу.
authentication-required = Автентифікація обов'язкова
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
        [log-out] Закрити застосунки й вийти?
        [confirm-device-type] Підтвердіть тип пристрою
       *[other] Виконати обрану дію зараз?
    }
confirm-body =
    { $action ->
        [restart] Перезавантаження системи
        [suspend] Призупинення системи
        [shutdown] Вимкнення системи
        [lock-screen] Блокування екрана
        [log-out] Завершення сеансу
        [enter-bios] Перехід до BIOS
       *[other] Виконання дії
    } за { $countdown } секунд.
