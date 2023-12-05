import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path_provider/path_provider.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:ui/main.dart';


class Settings extends StatefulWidget {
  const Settings({super.key});

  @override
  _SettingsState createState() => _SettingsState();


}


class _SettingsState extends State<Settings> {

  final TextEditingController _controller = TextEditingController();

  void inState() {
    super.initState();
    _controller.addListener(() {
      final String text = _controller.text.toLowerCase();
      _controller.value = _controller.value.copyWith(
        text: text,
        selection: TextSelection(baseOffset: text.length, extentOffset: text.length),
        composing: TextRange.empty
      );
    });
  }

  void dispose() {
    _controller.dispose(); // Wichtig, um Speicherlecks zu vermeiden
    super.dispose();
  }


  Future<void> requestPermissionAndCreateFile() async {
    var status = await Permission.storage.status;
    if (!status.isGranted) {
      status = await Permission.storage.request();
      if (status.isGranted) {
        await createFile();
      }
    } else {
      await createFile();
    }
  }

  Future<void> createFile() async {
    try {
      File file = File("${_controller.text}/PogSettings.txt");
      await file.writeAsString('Dies ist der Inhalt der Textdatei.');
      print('Textdatei erstellt: ${file.path}');
    } catch (e) {
      // wird ausgefüht wenn ein fehler auftritt
      print('Fehler beim Erstellen der Datei: $e');
    }
  }


  List _items = [];

  Future<void> readJason() async {
    final String respons = await rootBundle.loadString('assets/sampel.json');
    final data = await json.decode(respons);
    setState(() {
      _items = data["settings"];
      print("kein ahnuung: ${_items[0]}");
    });
  }

  Future<void> editJason() async {
    final String data = await rootBundle.loadString('assets/sampel.json');
    final edit = await json.decode(data);
    
  }

  bool hasRun = false;

  void runer () {
    if (!hasRun) {
      readJason();
      hasRun = true;
    }
  }



  @override
  Widget build(BuildContext context){
    
    return Scaffold(
      appBar: AppBar(  
        iconTheme: IconThemeData(
          color: Colors.black
        ),
        flexibleSpace: Container(color: Colors.amber),
        title: const Text(
          "Pogbilder - Settings", 
          style: TextStyle(
          color: Colors.black
            ) 
          ),
      ),
      body: Column(
        children: [
          Row(
                children: [
                  Container(
                    height: 45,
                    width: 120,
                    color: Colors.white38,
                    margin: EdgeInsets.all(10),
                    child: Align(
                      child: Text(
                        "File : ",
                        style: TextStyle(
                          fontSize: 20,
                          ),
                      ),
                    )
                  ),
                  Container(
                color: Colors.white24,
                width: 250,
                child:  TextField(
                  controller: _controller,
                  decoration: InputDecoration(
                     border: OutlineInputBorder(
                    ),
                    focusColor: Colors.black,
                    labelStyle: TextStyle(color: Colors.black)
                  ),
                ),
              ),
              
              Container(
                margin: EdgeInsets.all(10),
                child: Align(
                    child: TextButton.icon(
                      onPressed: () {
                        if (_controller.text.isNotEmpty) {
                          requestPermissionAndCreateFile();
                        } else {
                          showDialog(context: context, builder:(context) => AlertDialog(
                            title: const Text('Error!'),
                              content: const Text('the path muss bee \ngiven to create a file!'),
                              actions: <Widget>[
                                TextButton(
                                  onPressed: () => Navigator.pop(context, 'OK'),
                                  child: const Text('OK'),
                                ),
                              ],
                          ),);
                        }
                      }, 
                      icon: Icon(Icons.file_open), 
                      label: Text("Create file \nin given path"),
                      )
                  ),
              )
              
                ],
              ),
          Row( 
            children: [
              Container(
                margin: EdgeInsets.all(10),
                height: 45,
                width: 120,
                color: Colors.white38,
                child:  const Align(
                  alignment: Alignment.center,
                  child: Text(
                    "Username: ",
                    style: TextStyle(
                    fontSize: 20,
                    ),
                  ),
                )
              ),
              Container(
                color: Colors.white24,
                width: 250,
                child:  TextField(
                  
                  decoration: InputDecoration(
                    hintText: "${_items[0]}",
                     border: OutlineInputBorder(
                    ),
                    focusColor: Colors.black,
                    labelStyle: TextStyle(color: Colors.black)
                  ),
                ),
                
              ),

              IconButton(
                onPressed: () {
                 
                }, 
                icon: Icon(Icons.save))
            ],
          ),
          IconButton(
            onPressed: () {
              readJason();
            }, 
            icon: Icon(Icons.print_outlined))
        ],
      ),
    );
  }
}