invalid-password = 密码错误，请重试。
authentication-required = 需要认证
cancel = 取消
authenticate = 认证
log-out = 登出
suspend = 挂起
restart = 重启
enter-bios = 重启到BIOS
shutdown = 关机
confirm = 确认
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
    现在 { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [enter-bios] { enter-bios }
        [log-out] 退出所有应用程序并 { log-out }
        *[other] 执行所选操作
    } ？
confirm-body = 
    系统将在 { $countdown } 秒后自动 { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [lock-screen] 锁定屏幕
        [log-out] { log-out }
        [enter-bios] { enter-bios }
        *[other] 执行所选操作
    } 。

