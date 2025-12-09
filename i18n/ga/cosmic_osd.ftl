invalid-password = Focal faire neamhbhailí. Bain triail eile as.
authentication-required = Fíordheimhniú riachtanach
cancel = Cealaigh
authenticate = Fíordheimhnigh
log-out = Logáil Amach
restart = Atosaigh
enter-bios = Iontráil BIOS
shutdown = Múchadh
confirm = Deimhnigh
confirm-button =
    { $action ->
        [restart] { restart }
        [suspend] { suspend }
        [shutdown] Múchadh
        [log-out] { log-out }
        [enter-bios] { enter-bios }
       *[other] { confirm }
    }
confirm-title =
    { $action ->
        [restart] { restart } anois?
        [suspend] { suspend } anois?
        [shutdown] { shutdown } anois?
        [enter-bios] { enter-bios } anois?
        [log-out] Scoir as na haipeanna go léir agus logáil amach anois?
        [confirm-device-type] Deimhnigh cineál na gléasra
       *[other] Cuir an gníomh roghnaithe i bhfeidhm anois?
    }
confirm-body =
    Cuirfear an córas { $action ->
        [restart] atosaigh
        [suspend] cuir ar fionraí
        [shutdown] múchadh
        [lock-screen] cuir an scáileán faoi ghlas
        [log-out] logáil amach
        [enter-bios] atosaigh sa BIOS
       *[other] ar an ngníomh roghnaithe i bhfeidhm
    } go huathoibríoch i gceann { $countdown } soicindí.
suspend = Cuir ar fionraí
sound-settings = Socruithe fuaime
headphones = Cluasáin
headset = Gléas cinn
