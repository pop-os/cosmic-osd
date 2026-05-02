cancel = Откажи
confirm = Потврди
log-out = Одјава
suspend = Обустави
restart = Поново покрени
invalid-password = Нетачна лозинка. Покушајте поново.
authentication-required = Потребна је идентификација
authenticate = Идентификуј
enter-bios = Уђи у БИОС
sound-settings = Подешавања звука
shutdown = Угаси
headphones = Слушалице
headset = Слушалице са микрофоном
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Искључи
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } сада?
        [suspend] { suspend } сада?
        [shutdown] { shutdown } сада?
        [enter-bios] { enter-bios } сада?
        [log-out] Затвори све програме и одјави се сада?
        [confirm-device-type] Потврди врсту уређаја
       *[other] Примени изабрану радњу сада?
    }
confirm-body =
    Систем ће { $action ->
        [restart] се поново покренути
        [suspend] се обуставити
        [shutdown] се искључити
        [lock-screen] закључати екран
        [log-out] се одјавити
        [enter-bios] се поново покренути у БИОС
       *[other] применити изабрану радњу
    } самостално за { $countdown } секунди.
