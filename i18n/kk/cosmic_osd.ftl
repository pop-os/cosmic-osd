cancel = Бас тарту
confirm = Растау
invalid-password = Жарамсыз пароль. Қайталап көріңіз.
authentication-required = Аутентификация қажет
authenticate = Аутентификациялау
log-out = Жүйеден шығу
suspend = Ұйықтату
restart = Қайта іске қосу
enter-bios = BIOS-қа кіру
sound-settings = Дыбыс баптаулары
shutdown = Сөндіру
headphones = Құлаққаптар
headset = Гарнитура
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Сөндіру
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] Қазір { restart } керек пе?
        [suspend] Қазір { suspend } керек пе?
        [shutdown] Қазір { shutdown } керек пе?
        [enter-bios] Қазір { enter-bios } керек пе?
        [log-out] Барлық қолданбаларды жауып, қазір шығу керек пе?
        [confirm-device-type] Құрылғы түрін растау
       *[other] Таңдалған әрекетті қазір іске асыру керек пе?
    }
confirm-body =
    Жүйе { $action ->
        [restart] қайта іске қосылады
        [suspend] ұйықтатылады
        [shutdown] сөндіріледі
        [lock-screen] экранды бұғаттайды
        [log-out] жүйеден шығады
        [enter-bios] BIOS-қа кіретіндей қайта іске қосылады
       *[other] таңдалған әрекетті іске асырады
    } автоматты түрде { $countdown } секундтан кейін.
