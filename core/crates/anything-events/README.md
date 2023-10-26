# Anything events

The internal server for handling all events

## Development notes

Trigger an event

```bash
grpcurl -plaintext -d '{"event":{"identifier":{"source_id":1234},"details":{"name":"happy"}}}' localhost:5556 events.Events/TriggerEvent
```
