# 1.0.0 (2026-07-24)


### Bug Fixes

* **audio:** accessing sink volume array when checking source volume ([4e60034](https://github.com/playtron-os/cosmic-osd/commit/4e60034f0574e079c5577097a8a1ebeb3e7acf96))
* cleanup osd using helper ([c548cb6](https://github.com/playtron-os/cosmic-osd/commit/c548cb6faac256c5aaa59e3c975dc4bed8e35b0b))
* ensure no input is blocked by the osd indicator ([d7b4c88](https://github.com/playtron-os/cosmic-osd/commit/d7b4c8851c53b5593344c6f701186a20f82be782))
* focus cancel button after surface gains focus ([9e5e330](https://github.com/playtron-os/cosmic-osd/commit/9e5e3309172575b2ef18a775ba1520986c8d9853))
* focus cancel button after surface gains focus ([#185](https://github.com/playtron-os/cosmic-osd/issues/185)) ([e2c680e](https://github.com/playtron-os/cosmic-osd/commit/e2c680ed1cf7271fb4abfcffb72ebfeac0d76f09))
* focus polkit dialog text input ([2d8a84a](https://github.com/playtron-os/cosmic-osd/commit/2d8a84a95e1459a19aa0d967bcecca1a6a5df139))
* handle activation when already prompting for confirmation ([cd3d568](https://github.com/playtron-os/cosmic-osd/commit/cd3d5681973b606437f672c45d526f2b622e61d8))
* handling of simultaneous surfaces and dummy surface ([7190b6a](https://github.com/playtron-os/cosmic-osd/commit/7190b6a99e5175d4b102e4dd550f8c88f7d01247))
* **headset:** if headset is chosen, force the port of the default source for the preffered headset profile ([aa8a7d5](https://github.com/playtron-os/cosmic-osd/commit/aa8a7d5a01489bbf56e33f45abcadf1e497d7757))
* **headset:** set source if headset selected ([c26ecb0](https://github.com/playtron-os/cosmic-osd/commit/c26ecb025ddd3c778bd39a37739dce507933d680))
* **libcosmic:** theme subscription ([8eb1210](https://github.com/playtron-os/cosmic-osd/commit/8eb1210dfac37d426a6bc86ae56b2a445ced3251))
* match volume icons to volume applet ([3063271](https://github.com/playtron-os/cosmic-osd/commit/30632719fee6e3a0fb10ccdea53052e8b97a9bc0))
* patch deps ([9f8f6a5](https://github.com/playtron-os/cosmic-osd/commit/9f8f6a55b294a78037621e10d582febc6bc16369))
* polkit agent not working with sockets ([cb9698c](https://github.com/playtron-os/cosmic-osd/commit/cb9698c8536df05b7086115a071a3d3838b245f2))
* start the object server on the com.system76.CosmicOsd connection ([1e82a99](https://github.com/playtron-os/cosmic-osd/commit/1e82a9914ae043fbbec73a00d30c7dee35a1986f))
* surface cleanup ([c57df29](https://github.com/playtron-os/cosmic-osd/commit/c57df29816e9647bfe85086eae301da9671c21d8))
* update iced ([9588584](https://github.com/playtron-os/cosmic-osd/commit/95885844d35468feb0e39a82c7b829371f606cd6))
* use xdg base dir to locate volume change file ([4970139](https://github.com/playtron-os/cosmic-osd/commit/4970139d875f9b28ea72d431e07ca3650a200470))


### Features

* add rpm deployment config and CI ([b6f33e5](https://github.com/playtron-os/cosmic-osd/commit/b6f33e56caea68490227b9b1c328bd96ed1f4c7d))
* apply transparency to the colors ([48b46c9](https://github.com/playtron-os/cosmic-osd/commit/48b46c9b367dff551499403fe237c9e23c6c95d7))
* avoid overlap with panel ([5d6faf9](https://github.com/playtron-os/cosmic-osd/commit/5d6faf9fd99a548774c0ee2ed1ff66e87c6500ce))
* blur for osd indicator surfaces ([4bfdb53](https://github.com/playtron-os/cosmic-osd/commit/4bfdb5311a144d9c6aff102dc251e5be93f7579e))
* confirm headphone / headset ([6fd6a53](https://github.com/playtron-os/cosmic-osd/commit/6fd6a53c606da5ff1bbb6f7101bcacd76f2d2267))
* display toggle ([1b2e475](https://github.com/playtron-os/cosmic-osd/commit/1b2e475b1616ea791aa998967f6317460aa373eb))
* enter-bios confirmation dialog ([fa52300](https://github.com/playtron-os/cosmic-osd/commit/fa52300823ffe0d0cf9764c473d9231642037710))
* improve handling of blur on creation and tracking of overlaps ([8d66849](https://github.com/playtron-os/cosmic-osd/commit/8d66849f7e015e1c9fc428852e4ddaed691cec2e))
* new progress bar animations & min button width fix ([cbc1e9c](https://github.com/playtron-os/cosmic-osd/commit/cbc1e9cbf3af02faf8caa471dddcb03b98cd5f55))
* osd indicator for touchpad toggle ([720742c](https://github.com/playtron-os/cosmic-osd/commit/720742cc5289d81fb50d65097109e8664b9d505a))
* **osd_indicator:** respect amplification config ([63cbfa8](https://github.com/playtron-os/cosmic-osd/commit/63cbfa873bdd9fb1eda335e37408948fc0058906))
* play sound on sink volume change ([789698e](https://github.com/playtron-os/cosmic-osd/commit/789698e6d074d96fc502a5ee8bffba679dbca8a8))
* popups for log out, restart, and shutdown ([9b4e2be](https://github.com/playtron-os/cosmic-osd/commit/9b4e2bebbf8cf39c49c6dfa021afbcf49e1fd777))
* **sound:** use cosmic-settings-daemon's varlink API ([61063ff](https://github.com/playtron-os/cosmic-osd/commit/61063ffbdb777f3305998eee2a31f4afd75f2cde))


### Performance Improvements

* optimized cosmic-freedesktop-icons ([93ac12d](https://github.com/playtron-os/cosmic-osd/commit/93ac12d186e4e69cc0decfdc259c04633420b69a))
* update freedesktop-icons to reduce memory usage ([49ab688](https://github.com/playtron-os/cosmic-osd/commit/49ab688d3e679077d9bfdb6ea8ccfb657102721b))
* update libcosmic for reduced memory usage ([2abc7ad](https://github.com/playtron-os/cosmic-osd/commit/2abc7adf59c20fdc0b7e6391518711e760014462))

# Changelog

All notable changes to this project will be documented in this file. See
[semantic-release](https://github.com/semantic-release/semantic-release) for
commit conventions; entries below are generated on release.
