import 'dart:math';

import 'package:pogbilder/pages/Login.dart';
import 'package:pogbilder/demo.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(
     MaterialApp(
      theme: ThemeData(scaffoldBackgroundColor: Color.fromARGB(255, 59, 59, 59)),
      title: 'pogbilder.de    | pre Build',
      home: Login(),
      
    ),
  );
}
