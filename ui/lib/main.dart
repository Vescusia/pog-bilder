
import 'package:flutter/material.dart';
import 'package:ui/pages/Login.dart';
import 'dart:convert';
import 'dart:io';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:permission_handler/permission_handler.dart';


void main() {

  runApp(
     MaterialApp(
      theme: ThemeData(scaffoldBackgroundColor: const Color.fromARGB(255, 59, 59, 59)),
      title: 'pogbilder.de    | pre Build',
      home: Login(),
    ),
  );
}
