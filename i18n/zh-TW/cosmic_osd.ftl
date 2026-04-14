invalid-password = 密碼無效。請再試一次。
authentication-required = 需要驗證
cancel = 取消
authenticate = 驗證
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
    { $action ->
        [restart] 立即{ restart }？
        [suspend] 立即{ suspend }？
        [shutdown] 立即{ shutdown }？
        [enter-bios] 立即{ enter-bios }？
        [log-out] 結束所有應用程式並立即登出？
        [confirm-device-type] 確認裝置類型
       *[other] 立即套用所選操作？
    }
confirm-body =
    系統將在 { $countdown } 秒後自動{ $action ->
        [restart] 重新啟動
        [suspend] 休眠
        [shutdown] 關閉電源
        [lock-screen] 鎖定螢幕
        [log-out] 登出
        [enter-bios] 重新啟動進入 BIOS
       *[other] 套用所選操作
    }。
suspend = 休眠
enter-bios = 進入 BIOS
sound-settings = 音效設定
headphones = 耳機
headset = 耳機麥克風
