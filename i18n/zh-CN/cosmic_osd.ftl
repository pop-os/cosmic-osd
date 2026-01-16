invalid-password = 密码错误，请重试。
authentication-required = 需要认证
cancel = 取消
authenticate = 认证
log-out = 登出
suspend = 待机
restart = 重启
enter-bios = 重启到 BIOS
sound-settings = 声音设置
shutdown = 关机
headphones = 耳机
headset = 耳麦
confirm = 确认
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] 关机
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    立即{ $action ->
        [restart] { restart }？
        [suspend] { suspend }？
        [shutdown] { shutdown }？
        [enter-bios] { enter-bios }？
        [log-out] 退出所有应用程序并登出？
        [confirm-device-type] 确认设备类型
       *[other] 执行所选操作？
    }
confirm-body =
    系统将在 { $countdown } 秒后自动{ $action ->
        [restart] 重启
        [suspend] 待机
        [shutdown] 关机
        [lock-screen] 锁定屏幕
        [log-out] 登出
        [enter-bios] 重启到 BIOS
       *[other] 执行所选操作
    }。
