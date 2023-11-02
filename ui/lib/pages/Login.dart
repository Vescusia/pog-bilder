import 'package:flutter/material.dart';
import 'settings.dart';
import 'package:flutter_chat_bubble/chat_bubble.dart';
import 'package:flutter_chat_bubble/bubble_type.dart';
import 'package:flutter_chat_bubble/clippers/chat_bubble_clipper_1.dart';
import 'dart:math';


class Login extends StatefulWidget {
  const Login({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _LoginState createState() => _LoginState();
}

// Daten der ChatMessage

class ChatMessage_display{

  final int type;

  /*

  type == 1 -> Textmessage
  type == 2 -> Bld
  type == 3 -> voice
  type == 4 -> mix

  */

  final String data;
  final String username;
  final int time;
  

  ChatMessage_display({required this.type, required this.data, required this.username, required this.time});

    @override
  String toString(){
    return data;
  }



  int getTime(){
    return time;
  }

  

}

class ChatMessageMeta extends ChatMessage_display{

  final String userID;
  final String IP_address;

  ChatMessageMeta({required this.IP_address, required this.userID, required super.type, required super.data, required super.username, required super.time});
  
}


class _LoginState extends State<Login> {
  final TextEditingController _textController = TextEditingController();
  List<ChatMessage_display> messages = [];

  void _sendMessage(String enteredText) {
    setState(() {
      messages.add(ChatMessage_display(type: 1, data: enteredText, username: 'Tim', time: DateTime.now().second));
    });
    _textController.clear();

    
  }


  @override
  Widget build(BuildContext context) {
    return Scaffold(
      
      // darstellung der Oberenleiste 
      appBar: AppBar(
        leading: Builder(
          builder: (BuildContext context) {
            return IconButton(
              icon: const Icon(Icons.settings),
              onPressed: () {
                Navigator.push(context, MaterialPageRoute(builder: (context) => const Settings()));
              },
            );
          },
        ),
        flexibleSpace: Container(
          color: const Color.fromARGB(255, 5, 70, 100),
        ),
        title: const Text("Pogbilder - Chat"),
      ),

      // darstellung der Text nachrichten
      body: Column(
        children: <Widget>[
          Expanded(
            child: ListView.builder(
              itemCount: messages.length,
              itemBuilder: (context, index) {
                return ChatBubble(
                  //fomr der Chatbubble
                  clipper: ChatBubbleClipper1(type: BubbleType.sendBubble),

                  //Postion der Textbubble vom der Bildschrimseie
                  alignment: Alignment.centerRight,
                  //platz zwischen wand und Textbubble
                  margin: const EdgeInsets.all(10),

                  //frabe der Textblase
                  backGroundColor: const Color.fromARGB(255, 26, 197, 228),
                  
                  // padding
                  //daten in der Textblase
                  child:  Column (textDirection: TextDirection.rtl ,children: [
                     Align(
                      alignment: Alignment.bottomRight,
                      widthFactor: max(messages[index].toString().length.toDouble()/9, messages[index].username.toString().length.toDouble()/9),
                      child: 
                        const Text(
                          "u",
                          style: TextStyle(color: Color.fromARGB(255, 248, 248, 248)),
                      ),
                    ),
                    Text(
                      messages[index].toString(),
                      style: const TextStyle(color: Color.fromARGB(255, 255, 255, 255)),
                      ),
                    Align(
                      alignment: Alignment.topRight,
                      widthFactor: max(messages[index].toString().length.toDouble()/9, TimeOfDay.now().toString().length.toDouble()/9),
                      child: Text(
                        TimeOfDay.now().toString(),
                        style: const TextStyle(color: Color.fromARGB(255, 255, 255, 255)),
                        ),
                    )

                  ],
                  )
                );
              },
            ),
          ),
          Padding(
            padding: const EdgeInsets.all(15.0),
            child: Row(
              children: <Widget>[
                Expanded(
                  child: TextField(
                    controller: _textController,
                    decoration: InputDecoration(
                      hintText: 'Nachricht eingeben',
                      border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(20.0),
                      ),
                      contentPadding: const EdgeInsets.all(12.0),
                    ),
                    onSubmitted: (value) {
                      _sendMessage(value);
                    },
                  ),
                ),
                const SizedBox(width: 12.0),
                IconButton(
                  icon: const Icon(Icons.send),
                  onPressed: () {
                    _sendMessage(_textController.text);
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

