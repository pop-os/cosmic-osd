invalid-password = 유효하지 않은 암호입니다. 다시 시도해주세요.
restart = 재시작
confirm = 확인
authentication-required = 인증 필요
authenticate = 인증
log-out = 로그아웃
enter-bios = BIOS 진입
sound-settings = 소리 설정
shutdown = 종료
headphones = 헤드폰
headset = 헤드셋
suspend = 절전
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] 종료
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-body =
    시스템이 { $countdown }초 후 자동으로 { $action ->
        [restart] 재시작
        [suspend] 절전
        [shutdown] 종료
        [lock-screen] 화면 잠금
        [log-out] 로그아웃
        [enter-bios] BIOS로 재시작
       *[other] 선택된 동작이 적용
    }됩니다.
confirm-title =
    { $action ->
        [restart] 지금 { restart } 할까요?
        [suspend] 지금 { suspend } 할까요?
        [shutdown] 지금 { shutdown } 할까요?
        [enter-bios] 지금 { enter-bios } 할까요?
        [log-out] 지금 모든 앱을 종료하고 로그아웃 할까요?
        [confirm-device-type] 장치 종류 확인
       *[other] 지금 선택한 동작을 적용할까요?
    }
cancel = 취소
