invalid-password = Invalid password. Please try again.
authentication-required = Authentication Required
cancel = Cancel
authenticate = Authenticate
log-out = Log Out
suspend = Suspend
restart = Restart
enter-bios = Enter BIOS
sound-settings = Sound settings
shutdown = Shutdown
headphones = Headphones
headset = Headset
confirm = Confirm
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Power off
        [log-out] { log-out }
        [enter-bios] { enter-bios }
        *[other] { confirm }
}
confirm-title =
    { $action ->
        [restart] { restart } now?
        [suspend] { suspend } now?
        [shutdown] { shutdown } now?
        [enter-bios] { enter-bios } now?
        [log-out] Quit all applications and log out now?
        [confirm-device-type] Confirm Device Type
        *[other] Apply the selected action now?
    }
confirm-body =
    The system will { $action ->
        [restart] restart
        [suspend] suspend
        [shutdown] power off
        [lock-screen] lock the screen
        [log-out] log out
        [enter-bios] restart into BIOS
        *[other] apply the selected action
    } automatically in { $countdown } seconds.
