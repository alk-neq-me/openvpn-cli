from sys import argv
from os import listdir
from subprocess import run




CONFIG: str     = "./config/"
COUNTRY: str    = CONFIG+"country/"
AUTH: str       = CONFIG+"auth"

def start(mode: str ="cli"):
    if mode != "cli":
        country_opt: int = argv.index("-c")
        id_opt: int      = argv.index("-i")
        country: str     = argv[country_opt+1]
        id_: str         = argv[id_opt+1]
    else:
        country: str = input("Country: ")
        id_: str     = input("ID: ")

    id_ = f"0{id_}" if len(id_) == 1 else id_
    try:
        run([
            "sudo",
            "openvpn",
            "--config",
            f"{COUNTRY}{country}-free-{id_}.protonvpn.net.udp.ovpn",
            "--auth-user-pass",
            f"{AUTH}"
        ])
    except KeyboardInterrupt:
        print("\nBye!");


def main():
    if len(argv) > 3:
        start(mode="flag")
    elif len(argv) == 2:
        if (argv[1] == "--list") or (argv[1] == "-l"):
            for list in listdir(COUNTRY):
                print("🌎", list.split(".protonvpn")[0].replace("-free-", "  "))
    else:
        start()


if __name__ == "__main__":
    main()
