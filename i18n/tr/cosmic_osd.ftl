invalid-password = Geçersiz şifre. Lütfen tekrar deneyiniz.
authentication-required = Kimlik Doğrulama Gerekli
cancel = İptal
authenticate = Doğrula
log-out = Oturumu Kapat
suspend = Askıya Al
restart = Yeniden Başlat
enter-bios = BIOS'a Gir
shutdown = Kapat
confirm = Onayla
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Gücü kapat
        [log-out] { log-out }
        [enter-bios] { enter-bios }
        *[other] { confirm }
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] { enter-bios }
        [log-out] Tüm uygulamalardan çık ve oturumu kapat
        *[other] Seçilen işlemi uygula
    } şimdi?
confirm-body = 
    Sistem { $countdown } saniye içinde otomatik olarak { $action ->
        [restart] yeniden başlatılacak
        [suspend] askıya alınacak
        [shutdown] kapanacak
        [lock-screen] ekranı kilitleyecek
        [log-out] oturumu kapatacak
        [enter-bios] BIOS'a girmek için yeniden başlatılacak
        *[other] seçilen işlemi uygulayacak
    }.