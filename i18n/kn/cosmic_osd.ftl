invalid-password = ಅಮಾನ್ಯ ಪಾಸ್‌ವರ್ಡ್. ದಯವಿಟ್ಟು ಮತ್ತೊಮ್ಮೆ ಪ್ರಯತ್ನಿಸಿ.
authentication-required = ದೃಢೀಕರಣದ ಅಗತ್ಯವಿದೆ
cancel = ರದ್ದುಗೊಳಿಸಿ
authenticate = ಪ್ರಮಾಣೀಕರಿಸಿ
log-out = ಲಾಗ್ ಔಟ್
restart = ಪುನಾರಂಭಿಸಿ
shutdown = ಸ್ಥಗಿತಗೊಳಿಸಿ
confirm = ದೃಢೀಕರಿಸಿ
confirm-button = {
    $action -> 
        [restart] { restart }
        [suspend] { suspend}
        [shutdown] ಪವರ್ ಆಫ್
        [log-out] { log-out }
        *[other] { confirm}
}
confirm-title = 
    { $action -> 
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] { shutdown }
        [log-out] ಎಲ್ಲಾ ಅಪ್ಲಿಕೇಶನ್‌ಗಳನ್ನು ಬಿಟ್ಟು ಲಾಗ್ ಔಟ್ ಆಗಿ
        *[other] ಆಯ್ಕೆ ಮಾಡಿದ ಕ್ರಿಯೆಯನ್ನು ಕಾರ್ಯಗತಗೊಳಿಸಿ
    } now?
confirm-body = 
    ಈ ಸಿಸ್ಟಮ್ { $action ->
        [restart] ಪುನಾರಂಭಿಸುತ್ತಿದೆ
        [suspend] ಅಮಾನತುಗೊಳಿಸುತ್ತದೆ
        [shutdown] ಸ್ಥಗಿತಗೊಳಿಸುತ್ತಿದೆ
        [lock-screen] ಸ್ಕ್ರೀನನ್ನು ಲಾಕ್ ಮಾಡುತ್ತದೆ
        [log-out] ಲಾಗ್ ಔಟ್ ಆಗುತ್ತದೆ
        *[other] ಆಯ್ಕೆ ಮಾಡಿದ ಕ್ರಿಯೆಯನ್ನು ಕಾರ್ಯಗತಗೊಳಿಸಿ
    } ಸ್ವಯಂಚಾಲಿತವಾಗಿ { $countdown } ಸೆಕೆಂಡುಗಳಲ್ಲಿ.
