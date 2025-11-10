confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] כיבוי
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
