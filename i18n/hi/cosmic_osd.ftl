invalid-password = अमान्य पासवर्ड। कृपया पुनः प्रयास करें।
authentication-required = प्रमाणीकरण आवश्यक है
cancel = रद्द करें
authenticate = प्रमाणित करें
log-out = लॉग आउट
restart = पुनः आरंभ करें
shutdown = बंद करें
confirm = पुष्टि करें
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] बंद करें
        [log-out] { log-out }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] सभी अनुप्रयोगों को बंद करें और लॉग आउट करें
       *[other] चयनित क्रिया लागू करें
    } अब?
confirm-body =
    प्रणाली { $action ->
        [restart] पुनः आरंभ करेगी
        [suspend] निलंबित करेगी
        [shutdown] पावर ऑफ करेगी
        [lock-screen] स्क्रीन लॉक करेगी
        [log-out] लॉग आउट करेगी
       *[other] चयनित क्रिया लागू करेगी
    } स्वचालित रूप से { $countdown } सेकंड में.
suspend = निलंबित करें
enter-bios = BIOS में प्रवेश करें
sound-settings = ध्वनि सेटिंग
headphones = हेडफ़ोन
headset = हेडसेट
