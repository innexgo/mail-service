# mail-service

`mail-service` is a reusable microservice that manages reminders, notifications, mail, and text.
It can be run in dummy mode without API keys, and will simply write notifcations to standardout.
## API Endpoints
* `mail_new`  
  * If connected to a proper client and database, the mail_new function will send a new mail using our chosen AWS while loging that a mail has been sent. 
  * If there is no client in the case of a dryrun, mail_new will instead print the mail in the standard output.
* `mail_view`
  * If run succefully, mail_view will query all the past emails that were sent. 
  * If not, it will return an error.
* `public/api_info`
  * Displays current version information for the service

### Current Status
Currently, mail-service is incomplete. 
It only has support for the dummy mode without API keys.

## Running mail-service

Todo...
