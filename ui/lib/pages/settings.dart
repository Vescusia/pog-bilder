import 'package:flutter/material.dart';

class Settings extends StatelessWidget {
  const Settings({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Settings"),
        flexibleSpace: Container(
          color: const Color.fromARGB(255, 6, 84, 121),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(8),
        children: <Widget>[
          Container(
            width: 300,
          ),
            const FormExample(var_hinttext: 'user-ID'),
          Container(
            width: 300,
          ),
            const FormExample(var_hinttext: 'user-name'),
          Container(
            width: 300,
          ),
            const FormExample(var_hinttext: 'IP-adress'),
          ButtonBar(
            alignment: MainAxisAlignment.spaceEvenly,
            children: [
              TextButton(onPressed: () {}, child: const Text('Save Data'))
            ],
          ),
        ],
      ),
    );
  }
}

class FormExample extends StatefulWidget {
  final String var_hinttext;

  // ignore: non_constant_identifier_names
  const FormExample({required this.var_hinttext, Key? key}) : super(key: key);

  @override
  State<FormExample> createState() => _FormExampleState();
}

class _FormExampleState extends State<FormExample> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          TextFormField(
            decoration: InputDecoration(
              hintText: widget.var_hinttext,
            ),
            validator: (String? value) {
              if (value == null || value.isEmpty) {
                return 'Please enter the requiert argument!';
              }
              return null;
            },
          )
        ],
      ),
    );
  }
}
