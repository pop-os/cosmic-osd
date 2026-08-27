# Testing

This document provides a regression testing checklist for the COSMIC OSD component. The checklist provides a starting point for Quality Assurance reviews.

## Checklist

- [ ] Volume hotkeys work
- [ ] Plugging a headset into a combo jack prompts to select the type
- [ ] Screen brightness can be adjusted in 5% increments with hotkeys, down to 1%
- [ ] Keyboard backlight brightness can be adjusted via hotkeys
- [ ] Airplane mode can be enabled and disabled via hotkey
- [ ] Touchpad can be disabled and enabled via hotkey
- [ ] Open the Log Out dialog; tab key works to navigate buttons
    - [ ] Spacebar on Cancel exits dialog
    - [ ] Esc key (without anything focused) also exits dialog
- [ ] A privileged action (`pkexec true`) opens the authentication dialog
    - [ ] The correct password authorizes the action
    - [ ] A wrong password shows an error and allows a retry
    - [ ] Cancel dismisses the dialog and the action is denied
    - [ ] With a fingerprint enrolled, swiping authorizes without typing anything
    - [ ] With a fingerprint enrolled, the password field is usable immediately,
          rather than after the reader times out
    - [ ] With a reader present but no fingerprints enrolled, the password still works
    - [ ] With no reader at all, the password still works

## Testing the polkit helper from a build tree

`cosmic-polkit-helper` is socket-activated, so it is started by systemd rather
than by `cosmic-osd`. To test a build without installing it, point a drop-in at
the build tree:

```sh
just build-release
sudo just rootdir=/ install
sudo systemctl daemon-reload
sudo systemctl enable --now cosmic-polkit-helper.socket
sudo mkdir -p /etc/systemd/system/cosmic-polkit-helper@.service.d
printf '[Service]\nExecStart=\nExecStart=%s/target/release/cosmic-polkit-helper\n' "$PWD" \
    | sudo tee /etc/systemd/system/cosmic-polkit-helper@.service.d/local.conf
sudo systemctl daemon-reload
```

Helper logs land in the journal: `journalctl -fu 'cosmic-polkit-helper@*'`. Set
`RUST_LOG=debug` in the drop-in for more detail.

Removing `/run/cosmic-osd/polkit-helper.socket` (`sudo systemctl stop
cosmic-polkit-helper.socket`) makes `cosmic-osd` fall back to polkit's own
helper, which is how to compare against upstream behaviour.
