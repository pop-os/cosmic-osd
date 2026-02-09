invalid-password = Kata sandi tidak valid. Silakan coba lagi.
cancel = Batalkan
authentication-required = Autentikasi Diperlukan
authenticate = Autentikasikan
log-out = Keluar
suspend = Hentikan
restart = Mulai ulang
enter-bios = Masuk ke BIOS
sound-settings = Pengaturan suara
shutdown = Matikan
headphones = Headphone
headset = Headset
confirm = Konfirmasikan
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Matikan daya
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } sekarang?
        [suspend] { suspend } sekarang?
        [shutdown] { shutdown } sekarang?
        [enter-bios] { enter-bios } sekarang?
        [log-out] Tutup semua aplikasi dan keluar sekarang?
        [confirm-device-type] Konfirmasi Jenis Perangkat
       *[other] Terapkan tindakan yang dipilih sekarang?
    }
confirm-body =
    Sistem akan { $action ->
        [restart] memulai ulang
        [suspend] menangguhkan
        [shutdown] mematikan
        [lock-screen] mengunci layar
        [log-out] keluar
        [enter-bios] memulai ulang ke BIOS
       *[other] menerapkan tindakan yang dipilih
    } secara otomatis dalam { $countdown } detik.
