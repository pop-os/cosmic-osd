invalid-password = Паролата е неправилна. Опитайте отново.
authentication-required = Изисква се идентификация
cancel = Отказване
authenticate = Удостоверяване
log-out = Изход
suspend = Приспиване
restart = Рестартиране
enter-bios = Влизане в BIOS
shutdown = Изключване
confirm = Потвърждаване
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Изключване
        [log-out] { log-out }
        [enter-bios] {enter-bios}
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] {enter-bios}
        [log-out] Спиране на всички програми и изход
        *[other] Прилагане на избраното действие
    } сега?
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

