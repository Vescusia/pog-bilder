# The pog_bilder Message Protocol
 The aim of the *pog_bilder* protocol is to be as simple as possible whilst providing exactly the functionality we need.
 This also has to apply to any inevitable revisions of this document.
 

# The Objects
The actual data will be serialized using [BSON](https://en.wikipedia.org/wiki/BSON), a more efficient version of JSON.
This section will describe the actual data-objects being sent in a JSON-like form. This is not the [protocol](#the-protocol).

### Message 
```
product Message := {
	sender: Sender,
	timestamp: int,  // Seconds since Unix-Epoch
	data: MessageBody
}
```

### Sender
```
product Sender := {
	uuid: u128,
	name: String
}
```

### MessageBody
```
sum MessageBody := {
	Image(Bytes),
	Text(String),
	File(String, Bytes)  // (filename, data)
}
```

### MessageRequest
```
product MessageRequest := {
	sender: Sender,
	since: int,  // Seconds since Unix-Epoch
}
```

# The Protocol
TCP is used as the transport layer. It will take care of error checking and handling. Also, it will split up the Messages into sendable segments.
 
## Protocol

### Initialization
> Client connects to server, sends a [`MessageRequest`](#messagerequest) to Server. The `since` field should be the last logon time of the Client.

> Server replies, sending all [`Message`'s](#message) sent (by other Clients) since the `since` field, up until now.

### Message Exchange
> Client sends [`Message`](#message) to Server.

> Server broadcasts [`Message`](#message) to other connected clients.

> repeat until [Disconnect](#disconnect)

### Disconnect
*no important events, simple TCP disconnect*

## Protocol Diagram
' * ' stands for "zero or more times"

| Server | Client | Phase |
|--|--|--|
|  |  [`MessageRequest`](#messagerequest)| [Initilization](#initialization) |
| [`Message`](#message)* |  | [Initilization](#initialization) |
| [`Message`](#message)* | [`Message`](#message)* | [Message Exchange](#message-exchange) |
| | *disconnect* | [Disconnect](#disconnect) |
