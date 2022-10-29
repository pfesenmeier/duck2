# test if can send push notifications locally
- with or without timer
- mobile / desktop

- https://alestic.com/2019/05/aws-delayed-sns-step-functions/

Probably stack:
- web lambda posts to sns topic (or queue?)
- queue called with DelaySeconds (up to 15 minutes)
https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html
- lambda pulls events off queue and sends web notification

- web api lambda sends message to sqs queue
- delay seconds sent to queue
- queue calls another lambda, which sends message to web worker




# Web server to serve push notifications
Or perhaps... just a lambda function
With Secrets inside secret manager

https://crates.io/crates/web-push

user sends request to endpoint

# Web frontend with webworker that receives notifications

Looks like no special requirements within Safari
https://developer.apple.com/documentation/usernotifications/sending_web_push_notifications_in_safari_and_other_browsers

Straightforward within trunk.rs > can use Links to reference rust files


