invalid-password = パスワードは正しくありません。もう一度入力してください。
authentication-required = 認証が必要です
cancel = キャンセル
authenticate = 認証
log-out = ログアウト
suspend = サスペンド
enter-bios = BIOSに入る
restart = 再起動
shutdown = シャットダウン
confirm = 確認する
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] 電源オフ
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    すぐに{ $action ->
        [restart] { restart }しますか？
        [suspend] { suspend }しますか？
        [shutdown] { shutdown }しますか？
        [enter-bios] { enter-bios }しますか？
        [log-out] 全てのアプリケーションを終了してログアウトしますか？
        [confirm-device-type] デバイスタイプを確認しますか？
       *[other] 選択した処理を実行しますか？
    }
confirm-body =
    システムは{ $countdown }秒後に自動的に{ $action ->
        [restart] 再起動
        [suspend] サスペンド
        [shutdown] シャットダウン
        [lock-screen] 画面ロック
        [log-out] ログアウト
        [enter-bios] 再起動しBIOS起動
       *[other] 選択した処理を実行
    }します。
sound-settings = サウンド設定
headphones = ヘッドホン
headset = ヘッドセット
