# The pog_bilder Message Protocol
 The aim of the *pog_bilder* protocol is to be as simple as possible whilst providing exactly the functionality we need.
 This also has to apply to any inevitable revisions of this document.
 

# The Objects
The actual data will be serialized using [Protocol Buffers](https://protobuf.dev/programming-guides/proto3/), matching the simple and efficient needs for this project.

This is not the [protocol](#the-protocol).
This section will describe the data-objects being sent in an abstract way.

**These are abstract definitions, please refer to the `messages.proto` file for concrete specifics.**

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
	uuid: u64,
	name: String
}
```

### MessageBody
```
sum MessageBody := {
	Image{image: Bytes},
	Text{text: String},
	BigFileOffer{filename: String, size: u64},
	BigFileAccept{timestamp: int},  // accept Offer with this timestamp (seconds since Unix-Epoch)
	BigFileData{timestamp: int, bytes: Bytes}  // send some Bytes belonging to the Offer with this timestamp (seconds since Unix-Epoch)
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

### Big File Exchange
> Client sends [`Message`](#message), with `data` = `BigFileOffer`, with the `size` = size in bytes of the file.

> Server may accept, sending a [`Message`](#message), with `data` = `BigFileAccept`, where the `timestamp` equals the one of the original offer.

> Client then has the responsibility to send as many `bytes`, using [`Message`](#message)'s with `data` = `BigFileData`, as promised in the original offer.

### Disconnect
*no important events, simple TCP disconnect*

If there are any `BigFile`'s with not all promised `bytes` received, they shall be discarded by the server.


## Protocol Diagram
' * ' stands for "zero or more times"

| Server                 | Client                              | Phase                                                                       |
|------------------------|-------------------------------------|-----------------------------------------------------------------------------|
|                        | [`MessageRequest`](#messagerequest) | [Initilization](#initialization)                                            |
| [`Message`](#message)* |                                     | [Initilization](#initialization)                                            |
| [`Message`](#message)* | [`Message`](#message)*              | [Message Exchange](#message-exchange)/[BigFileExchange](#big-file-exchange) |
|                        | *disconnect*                        | [Disconnect](#disconnect)                                                   |
