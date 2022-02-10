# Group Activity Bot

Group Activity Bot (GAB) is a Telgram bot that will keep track of user activities in groups, e.g.

<img width="1155" alt="Screenshot 2022-02-10 at 13 57 41" src="https://user-images.githubusercontent.com/16304728/153413001-c55f3f46-e0f1-4661-9591-a9e1ed505892.png">

**GAB only works in group and must be a group admin in order to receive each user message in a group**

## Features
|Command|Action|
|:-|:-|
|`/groupstats`|Returns a message containing the total number of messages exchanged in the group and the percentege of messages per user, with nice emojis for the top 3 users|
|`/userstats <username>`|Returns a message containing the percentage of messages of a specific user in a group|
|`/statsfile`|Returns a .csv file containing `(message timestamp, username)` for all the messages exchanged in a group, this is helpful to plot graphs and other analytics with other softwares|

## How to run
I personally run rust bots on my raspberry and let systemd handle them

### Configuration
Create a service file, e.g. `/etc/systemd/system/bot.service`
```
[Unit]
Description=Telegram Bot Service
After=network.target

[Service]
Type=simple
User=<user>
Group=<group>
Restart=always
RestartSec=10
ExecStart=/usr/local/bin/bot
Environment="TELOXIDE_TOKEN=<token>"

[Install]
WantedBy=multi-user.target
```

Copy the bot binary in `/usr/local/bin/`

### Run
Enable the service so that it boots on reboot and start it
```
$ sudo systemctl enable bot.service
$ sudo systemctl start bot.service
```