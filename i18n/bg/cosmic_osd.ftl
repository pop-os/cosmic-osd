invalid-password = Паролата е неправилна. Опитайте отново.
authentication-required = Изисква се идентификация
cancel = Отказване
authenticate = Удостоверяване
log-out = Изход
suspend = Приспиване
restart = Рестартиране
enter-bios = Влизане в BIOS
sound-settings = Настройки на звука
shutdown = Изключване
headphones = Слушалки
headset = Слушалки с микрофон
confirm = Потвърждаване
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Изключване
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } сега?
        [suspend] { suspend } сега?
        [shutdown] { shutdown } сега?
        [enter-bios] { enter-bios } сега?
        [log-out] Спиране на всички програми и изход сега?
        [confirm-device-type] Потвърдете вида на устройството
       *[other] Прилагане на избраното действие сега?
    }
confirm-body =
    Системата ще { $action ->
        [restart] се рестартира
        [suspend] се приспи
        [shutdown] се изключи
        [lock-screen] се заключи
        [log-out] излезе от сесията
        [enter-bios] се рестартира от BIOS
       *[other] приложи избраното действие
    } автоматично след { $countdown } секунди.
