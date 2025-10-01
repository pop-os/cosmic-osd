invalid-password = 無效的密碼。 請再試一次。
authentication-required = 需要認證
cancel = 取消
authenticate = 認證
log-out = 登出
restart = 重新啟動
shutdown = 關機
confirm = 確認
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] 關閉電源
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    現在{ $action ->
        [restart] { restart }嗎？
        [suspend] { suspend }嗎？
        [shutdown] { shutdown }嗎？
        [enter-bios] { enter-bios }嗎？
        [log-out] 關閉所有應用程式並登出嗎？
        [confirm-device-type] 確認裝置類型
       *[other] 立即執行選定的操作嗎？
    }
confirm-body =
    系統將在 { $countdown } 秒後自動 { $action ->
        [restart] 重新啟動
        [suspend] 暫停
        [shutdown] 關機
        [lock-screen] 鎖定螢幕
        [log-out] 登出
       *[other] 執行選定的操作
    }。
suspend = 暫停
