invalid-password = كَلِمَةُ المُرُورِ غَيرُ صَحِيحَة. مِن فَضلِكَ حَاوِل مَرَّةً أُخرَى.
authentication-required = الاسْتِيثَاقُ مُتَطَلَّب
cancel = ألغِ
authenticate = اِستَوثِق
log-out = اِخرُج
suspend = عَلِّق
restart = أعِدِ التَّشغِيل
# FIX: Find better translation (https://github.com/pop-os/cosmic-osd/issues/101).
enter-bios = اِنتَقِل إلَى البايوس
sound-settings = إعدَادَاتُ الصَّوت
shutdown = أوقِفِ التَّشغيل
headphones = سَمَّاعَاتُ رَأس
headset = سَمَّاعَاتُ رَأسٍ بِلَاقِط
confirm = أكِّد
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] إيقاف التشغيل
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } الآن؟
        [suspend] { suspend } الآن؟
        [shutdown] { shutdown } الآن؟
        [enter-bios] { enter-bios } الآن؟
        [log-out] إغلاق جميع التطبيقات وتسجيل الخروج الآن؟
        [confirm-device-type] تأكيد نوع الجهاز
       *[other] تطبيق الإجراء المحدد الآن؟
    }
confirm-body =
    سيقوم النظام بـ { $action ->
        [restart] إعادة التشغيل
        [suspend] التعليق
        [shutdown] إيقاف التشغيل
        [lock-screen] قفل الشاشة
        [log-out] تسجيل الخروج
        [enter-bios] إعادة التشغيل في BIOS
       *[other] تطبيق الإجراء المحدد
    } تلقائيًا في غضون { $countdown } ثانية.
