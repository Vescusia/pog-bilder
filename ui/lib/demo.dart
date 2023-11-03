import 'package:flutter/material.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
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
  GlobalKey _childKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Container(
          key: _childKey,
          width: 100,
          height: 200,
          color: Colors.blue,
          child: Center(
            child: Text('Widget mit dynamischer Höhe'),
          ),
        ),
        ElevatedButton(
          onPressed: () {
            double height = getHeight(_childKey);
            print('Höhe des Widgets: $height');
          },
          child: Text('Höhe des Widgets abrufen'),
        ),
      ],
    );
  }

  double getHeight(GlobalKey key) {
    final RenderBox renderBox = key.currentContext!.findRenderObject() as RenderBox;
    return renderBox.size.height;
  }
}
