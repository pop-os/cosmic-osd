cancel = Têk bibe
confirm = Bipejirîne
restart = Ji nû ve bide destpêkirin
shutdown = Vemrîne
suspend = Rawestîne
invalid-password = Borînpeyva nederbasdar. Tika ye dîsa hewl bide.
authentication-required = Pêdivî bi rastandinê heye
authenticate = Rastandinê bike
log-out = Derkeve
enter-bios = Têkeve BIOS
sound-settings = Sazkariyên dengê
headphones = Guhik
headset = Guhik
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Vemrîne
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] Niha { restart }?
        [suspend] Niha { suspend }?
        [shutdown] Niha { shutdown }?
        [enter-bios] Niha { enter-bios }?
        [log-out] Hemû sepanan bigire û niha derkeve?
        [confirm-device-type] Cûreya amûrê bipejirîne
       *[other] Çalakiya hilbijartî niha bisepîne?
    }
confirm-body =
    Pergal wê xweber were { $action ->
        [restart] jinûve destpêkirin
        [suspend] rawestandin
        [shutdown] vemirandin
        [lock-screen] kilîtkirin
        [log-out] derketin
        [enter-bios] jinûvedestpêkkirin di hundir BIOS de
       *[other] çalakiya hilbijartî bisepîne
    } di { $countdown } çirke de.
