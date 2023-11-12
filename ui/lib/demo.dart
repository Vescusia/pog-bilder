import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: Text('JSON Beispiel'),
        ),
        body: MyWidget(),
      ),
    );
  }
}

class MyWidget extends StatefulWidget {
  @override
  _MyWidgetState createState() => _MyWidgetState();
}

class _MyWidgetState extends State<MyWidget> {
  Map<String, dynamic> jsonData = {};

  @override
  void initState() {
    super.initState();
    // Beim Start die Daten aus der JSON-Datei lesen
    readDataFromJson();
  }

  void readDataFromJson() {
    try {
      
      String filePath = "H:/Flutter/Pogbilder UI/pog-bilder/ui/deine_datei.json";
      File file = File(filePath);

      // Lesen des JSON-Strings aus der Datei
      Map<String, dynamic> data = jsonDecode(jsonString);
      String jsonString = file.readAsStringSync();

      // Konvertieren des JSON-Strings in ein Dart-Map-Objekt
      

      setState(() {
        jsonData = data;
      });
    } catch (e) {
      print('Fehler beim Lesen der Daten: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: <Widget>[
        ElevatedButton(
          onPressed: () {
            // Beim Knopfdruck die Daten in der Konsole ausgeben
            print('JSON-Daten: $jsonData');
          },
          child: Text('Daten ausgeben'),
        ),
      ],
    );
  }
}
