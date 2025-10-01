invalid-password = 密码错误，请重试。
authentication-required = 需要认证
cancel = 取消
authenticate = 认证
log-out = 登出
suspend = 挂起
restart = 重启
enter-bios = 重启到BIOS
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
    现在{ $action ->
        [restart] { restart }吗？
        [suspend] { suspend }吗？
        [shutdown] { shutdown }吗？
        [enter-bios] { enter-bios }吗？
        [log-out] 退出所有应用程序并登出吗？
        [confirm-device-type] 确认设备类型
       *[other] 执行所选操作吗？
    }
confirm-body =
    系统将在 { $countdown } 秒后自动{ $action ->
        [restart] 重启
        [suspend] 挂起
        [shutdown] 关机
        [lock-screen] 锁定屏幕
        [log-out] 登出
        [enter-bios] 重启到BIOS
       *[other] 执行所选操作
    }。
