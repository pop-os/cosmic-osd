invalid-password = كَلِمَةُ المُرُورِ غَيرُ صَحِيحَة. مِن فَضلِكَ حَاوِل مَرَّةً أُخرَى.
authentication-required = الاسْتِيثَاقُ مُتَطَلَّب
cancel = ألغِ
authenticate = اِستَوثِق
log-out = اِخرُج
suspend = عَلِّق
restart = أعِدِ التَّشغِيل
# FIX: Find better translation (https://github.com/pop-os/cosmic-osd/issues/101).
enter-bios =  اِنتَقِل إلَى البايوس
sound-settings =  إعدَادَاتُ الصَّوت
shutdown = أوقِفِ التَّشغيل
headphones = سَمَّاعَاتُ رَأس
headset = سَمَّاعَاتُ رَأسٍ بِلَاقِط
confirm = أكِّد
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] { log-out }
        [enter-bios] { enter-bios }
        *[other] { confirm }
}
confirm-title = 
    { $action ->
        [restart] أمُتَأكِّدٌ مِن إعادةِ التَّشغيلِ الآن؟
        [suspend] أمُتَأكِّدٌ مِن التَّعليقِ الآن؟
        [shutdown] أمُتَأكِّدٌ مِن إيقافِ التَّشغيلِ الآن؟
        [enter-bios] أمُتَأكِّدٌ مِن الاِنتِقَالِ إلَى البايوسِ الآن؟
        [log-out] أمُتَأكِّدٌ مِن إنهَاءِ كَافَّةِ التَّطبِيقَاتِ وَالخُرُوجِ الآن؟
        [confirm-device-type] أكِّدْ نَوعَ الجِّهَاز
        *[other] أمُتَأكِّدٌ مِن تَطبيقِ الإجرَاءِ المُحَدَّدِ الآن؟
    }
confirm-body = {
    $action ->
        [restart] سَيُعادُ تشغيلَ النِّظامِ
        [suspend] سَيُعَلَّقُ النِّظامَ
        [shutdown] سَيُوقَفُ تشغيلَ النِّظامِ
        [lock-screen] سَتُقفَلُ الشَّاشَةَ
        [log-out] سَيُخرَجُ المُستَخدِمَ
        [enter-bios] سَيُعادُ تشغيلَ النِّظامِ إلَى البايوسِ
        *[other] سَيُطَبَّقُ الإجرَاءَ المُحَدَّدُ
    } تِلْقائِيًّا خِلالَ { $countdown ->
        [zero] لَحظَة.
        [one] ثانيةٍ واحِدَة.
        [two] ثانِيَتَين.
        [few] {$countdown} ثوانٍ.
        [many] {$countdown} ثانِية.
        *[other] {$countdown} ثانِية.
    }.
