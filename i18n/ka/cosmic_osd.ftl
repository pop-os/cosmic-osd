cancel = გაუქმება
restart = გადატვირთვა
suspend = ძილი
authenticate = ავთენტიკაცია
headphones = ყურსასმენი
headset = მიკროფონიანი ყურსასმენი
confirm = დადასტურება
authentication-required = საჭიროა ავთენტიკაცია
log-out = ანგარიშიდან გასვლა
enter-bios = BIOS-ში შესვლა
sound-settings = ხმის მორგება
invalid-password = პაროლი არასწორია. თავიდან სცადეთ.
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] გამორთვა
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } ახლავე?
        [suspend] { suspend } ახლავე?
        [shutdown] { shutdown } ახლავე?
        [enter-bios] { enter-bios } ახლავე?
        [log-out] დავხურო ყველა აპლიკაცია და გავიდე ანგარიშიდან ახლავე?
        [confirm-device-type] მოწყობილობის ტიპის დადასტურება
       *[other] გადავატარო მონიშნული ქმედება ახლა?
    }
confirm-body =
    სისტემა { $action ->
        [restart] გადაიტვირთება
        [suspend] დაიძინებს
        [shutdown] გამოირთვება
        [lock-screen] ჩაკეტავს ეკრანს
        [log-out] გავა ანგარიშიდან
        [enter-bios] გადაიტვირთება BIOS-ში
       *[other] გამოიყენებს არჩეულ ქმედებს
    } ავტომატურად { $countdown } წამში.
shutdown = გამორთვა
