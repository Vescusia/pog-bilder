
import 'package:flutter/material.dart';
import 'package:ui/pages/Login.dart';

void main() {
  runApp(
     MaterialApp(
      theme: ThemeData(scaffoldBackgroundColor: const Color.fromARGB(255, 59, 59, 59)),
      title: 'pogbilder.de    | pre Build',
      home: Login(),
      
    ),
  );
}
