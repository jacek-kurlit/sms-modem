# sms-modem

My small app for sending SMS via USB modem.
Currently working on Alcatel USB dongle

## Roadmap

- [x] Make sure to use db ids for relations between repositories.
For example if contact were change and it was assigned to some group(s)
- [x] Try to replace all repositories with one generic implementation.
- [x] Make sms_cli to use static references to configuration and db
to be reused across whole app
- [ ] make sms_api to handle Mock and Alcatel providers
