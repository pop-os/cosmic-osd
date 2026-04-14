cancel = Sefsex
confirm = Sentem
restart = Ales asekker
suspend = Ḥbes di leɛḍil
invalid-password = Awal uffir d armeɣtu. Ttxil-k·m, ɛreḍ tikkelt nniḍen.
authentication-required = Asesteb yettwasra
authenticate = Sesteb
log-out = Ffeɣ
enter-bios = Kcem ɣer BIOS
sound-settings = Iɣewwaṛen n yimesli
headphones = Isawaḍen n uqerru
headset = Kask
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Sexsi
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } Tura?
        [suspend] { suspend } Tura?
        [shutdown] { shutdown } Tura?
        [enter-bios] { enter-bios } Tura?
        [log-out] Ffeɣ akk isnasen sakin teffɣeḍ tura?
        [confirm-device-type] Sentem anaw n yibenk
       *[other] Snes tigawt yettwafernen akka tura?
    }
confirm-body =
    Anagraw ad { $action ->
        [restart] yales asenker
        [suspend] yettwaḥbes di leɛḍil
        [shutdown] yexsi
        [lock-screen] isekkeṛ agdil
        [log-out] yeffeɣ
        [enter-bios] yales tanekra di BIOS
       *[other] yesnes tigawt yettwafernen
    } s wudem awurman di { $countdown } n tsinin.
shutdown = Sexsi
