# Open VPN Project


## Usage - Avaliable only with Rust 🦀

### Fast Mode
```sh
# with fast mode
rustc vpn.rs && ./vpn
```

### Custom Mode
```sh
# with custom mode
# Options
# -c <country_name>
# -i <country_number>
rustc vpn.rs && ./vpn -c jp -i 1
```

### List Mode
```sh
# with list mode
# show avaliable countries and their numbers
rustc vpn.rs && ./vpn --list
```

---

## Usage - Avaliable only with Python 🐍

### Custom Mode
```sh
# with custom mode
# Options
# -c <country_name>
# -i <country_number>
python vpn.py -c jp -i 1
```

### Optional Mode
```sh
# with optional mode
python vpn.py
```

### List Mode
```sh
# with list mode
# show avaliable countries and their numbers
python vpn.py --list
```
