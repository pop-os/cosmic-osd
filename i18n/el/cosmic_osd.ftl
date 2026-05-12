cancel = Ακύρωση
restart = Επανεκκίνηση
sound-settings = Ρυθμίσεις ήχου
shutdown = Τερματισμός
headphones = Ακουστικά
headset = Ακουστικά
confirm = Επιβεβαίωση
log-out = Αποσύνδεση
suspend = Αναστολή
invalid-password = Μη έγκυρος κωδικός πρόσβασης. Δοκιμάστε ξανά.
confirm-title =
    { $action ->
        [restart] { restart } τώρα;
        [suspend] { suspend } τώρα;
        [shutdown] { shutdown } τώρα;
        [enter-bios] { enter-bios } τώρα;
        [log-out] Έξοδος από όλες τις εφαρμογές και αποσύνδεση τώρα;
        [confirm-device-type] Επιβεβαίωση τύπου συσκευής
       *[other] Εφαρμογή της επιλεγμένης ενέργειας τώρα;
    }
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Τερματισμός
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
enter-bios = Είσοδος στο BIOS
confirm-body =
    Θα εκτελεστεί αυτόματα { $action ->
        [restart] επανεκκίνηση
        [suspend] αναστολή
        [shutdown] τερματισμός
        [lock-screen] κλείδωμα της οθόνης
        [log-out] αποσύνδεση
        [enter-bios] επανεκκίνηση στο BIOS
       *[other] η επιλεγμένη ενέργεια
    } του συστήματος σε { $countdown } δευτερόλεπτα.
authentication-required = Απαιτείται πιστοποίηση
authenticate = Πιστοποίηση
