invalid-password = Invalid password. Please try again.
authentication-required = Authentication Required
cancel = Cancel
authenticate = Authenticate
log-out = Log Out
restart = Restart
shutdown = Shutdown
confirm = Confirm
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] Power off
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] Quit all applications and log out
        *[other] Apply the selected action
    } now?
confirm-body = 
    The system will { $action ->
        [restart] restart
        [suspend] suspend
        [shutdown] power off
        [lock-screen] lock the screen
        [log-out] log out
        *[other] apply the selected action
    } automatically in { $countdown } seconds.

