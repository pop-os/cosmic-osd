confirm = ਤਸਦੀਕ
cancel = ਰੱਦ ਕਰੋ
log-out = ਲਾਗ ਆਉਟ
suspend = ਸਸਪੈਂਡ
restart = ਮੁੜ-ਚਾਲੂ
shutdown = ਬੰਦ ਕਰੋ
invalid-password = ਪਾਸਵਰਡ ਗਲਤ ਹੈ। ਫ਼ੇਰ ਕੋਸ਼ਿਸ਼ ਕਰੋ।
authentication-required = ਪਰਮਾਣੀਕਰਨ ਚਾਹੀਦਾ ਹੈ
authenticate = ਪਰਮਾਣਿਤ
enter-bios = BIOS ਵਿੱਚ ਜਾਓ
sound-settings = ਸਾਊਂਡ ਸੈਟਿੰਗਾਂ
headphones = ਹੈੱਡਫ਼ੋਨ
headset = ਹੈੱਡਸੈੱਟ
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] ਬੰਦ ਕਰੋ
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] ਹੁਣੇ { restart } ਕਰਨਾ ਹੈ?
        [suspend] ਹੁਣੇ { suspend } ਕਰਨਾ ਹੈ?
        [shutdown] ਹੁਣੇ { shutdown } ਕਰਨਾ ਹੈ?
        [enter-bios] ਹੁਣੇ { enter-bios } ਵਿੱਚ ਜਾਣਾ ਹੈ?
        [log-out] ਸਾਰੀਆਂ ਐਪਲੀਕੇਸ਼ਨ ਚੋਂ ਬਾਹਰ ਜਾ ਕੇ ਲਾਗਆਉਟ ਕਰਨਾ ਹੈ?
        [confirm-device-type] ਡਿਵਾਈਸ ਕਿਸਮ ਦੀ ਤਸਦੀਕ
       *[other] ਚੁਣੀ ਕਾਰਵਾਈ ਨੂੰ ਹੁਣੇ ਲਾਗੂ ਕਰਨਾ ਹੈ?
    }
confirm-body =
    ਸਿਸਟਮ ਨੂੰ { $countdown } ਸਕਿੰਟਾਂ ਵਿੱਚ ਆਪਣੇ-ਆਪ{ $action ->
        [restart] ਮੁੜ-ਚਾਲੂ
        [suspend] ਸਸਪੈਂਡ
        [shutdown] ਬੰਦ
        [lock-screen] ਸਕਰੀਨ ਨੂੰ ਲਾਕ
        [log-out] ਲਾਗ ਆਉਟ
        [enter-bios] BIOS ਲਈ ਮੁੜ-ਚਾਲੂ
       *[other] ਚੁਣੀ ਕਾਰਵਾਈ ਨੂੰ ਲਾਗੂ
    } ਕੀਤਾ ਜਾਵੇਗਾ।
