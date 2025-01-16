invalid-password = パスワードは正しくありません。もう一度入力してください。
authentication-required = 認証が必要です。
cancel = キャンセル
authenticate = 認証
log-out = ログアウト
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
        *[other] { confirm}
}
confirm-title =
    今{ $action ->
        [restart] { restart }しますか？
        [suspend] { suspend }しますか？
        [shutdown] 電源を切りますか？
        [log-out] アプリケーションをすべて閉じてログアウトしますか？
        *[other] 選択した処理を実行しますか？
    }
confirm-body = 
    { $countdown }秒後にシステムは自動的に{ $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [lock-screen] { lock-screen }
        [log-out] { log-out }
        *[other] 選択した処理を
    }します。

