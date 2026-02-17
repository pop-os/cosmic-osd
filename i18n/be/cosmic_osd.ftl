confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Выключыць
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
cancel = Скасаваць
invalid-password = Няправільны пароль. Паспрабуйце яшчэ раз.
authentication-required = Патрабуецца аўтэнтыфікацыя
authenticate = Аўтэнтыфікацыя
log-out = Выйсці
suspend = Прыпыніць
restart = Перазапусціць
enter-bios = Увайсці ў BIOS
sound-settings = Налады гуку
shutdown = Выключэнне
headphones = Навушнікі
headset = Гарнітура
confirm = Пацвердзіць
confirm-title =
    { $action ->
        [restart] { restart } зараз?
        [suspend] { suspend } зараз?
        [shutdown] { shutdown } зараз?
        [enter-bios] { enter-bios } зараз?
        [log-out] Закрыць усе праграмы і выйсці зараз?
        [confirm-device-type] Пацвердзіце тып прылады
       *[other] Прымяніць выбранае дзеянне зараз?
    }
confirm-body =
    Сістэма { $action ->
        [restart] будзе перазапушчана
        [suspend] будзе прыпынена
        [shutdown] будзе выключана
        [lock-screen] будзе заблакіравана
        [log-out] выйдзе
        [enter-bios] будзе перазапушчана ў BIOS
       *[other] прыменіць выбранае дзеянне
    } аўтаматычна праз { $countdown } секунд.
