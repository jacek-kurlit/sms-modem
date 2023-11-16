# sms-modem

My small app for sending SMS via USB modem.
Currently working on Alcatel USB dongle

## Roadmap

- [x] Make sure to use db ids for relations between repositories.
For example if contact were change and it was assigned to some group(s)
- [x] Try to replace all repositories with one generic implementation.
- [x] Make sms_cli to use static references to configuration and db
to be reused across whole app
- [x] make sms_api to handle Mock and Alcatel providers
- [x] make sms_api to handle sending to contacts and groups with plain text and templates
- [ ] Add ability to replace all contacts with values from csv
- [ ] Add ability to read received messages
- [ ] Add scheduler to plan when to send sms (hard, requires some service in background)
