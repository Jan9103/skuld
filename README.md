# Skuld

A [suckless](https://suckless.org/philosophy/) job sheduler inspired by [cron](https://en.wikipedia.org/wiki/Cron).

Cron was a greek god of time; Skuld was a germanic goddes of time (future).

## Usage

Example:
```bash
# send a notification about the time every hour (at 0 minutes)
echo '* * * 0 notify-send "$(date)"' | skuld

# rebuild your project every 5 minutes
echo '* * * /5 make all' | skuld

# update the system every monday at 1am
# (please don't run this if you don't know what you are doing)
cat | skuld <<EOF
* 1 1 0 apt update -y && apt upgrade -y
* 1 1 0 flatpak update --system -y --noninteractive
EOF

# when running from a init-system this might be more apropriate
cat /etc/skuld-config | skuld
```

Send a newline-seperated list of entries to its stdin. done.

Entry format:
```
* * * * /bin/sh command
| | | |
| | | +- minute (0-59)
| | +- hour (0-23)
| +- day of week (1-7, 1 = monday)
+- day of month (1-31)
```

a time can be:
- `*`: any
- a number: at that exact time
- `/` + a number: whenever it is dividable by this (`/5` = every 5 min)
