invalid-password = パスワードは正しくありません。もう一度入力してください。
authentication-required = 認証が必要です。
cancel = キャンセル
authenticate = 認証
log-out = ログアウト
suspend = サスペンド
enter-bios = BIOSに入る
restart = 再起動
shutdown = シャットダウン
confirm = 確認
cancel = キャンセル
confirm-button = {
    $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] 電源オフ
        [log-out] { log-out }
        [enter-bios] {enter-bios}
        *[other] { confirm}
}
confirm-title =
    すぐに{ $action ->
        [restart] { restart }しますか？
        [suspend] { suspend }しますか？
        [shutdown] 電源を切りますか？
        [enter-bios] BIOSに入りますか？
        [log-out] アプリケーションをすべて閉じてログアウトしますか？
        *[other] 選択した処理を実行しますか？
    }
confirm-body = 
    { $countdown }秒後にシステムは自動的に{ $action ->
        [restart] { restart }します。
        [suspend] { suspend }します。
        [shutdown] { shutdown }します。
        [lock-screen] { lock-screen }します。
        [log-out] { log-out }します。
        [enter-bios] 再起動して、BIOSに入ります。
        *[other] 選択した処理をします。
    }

