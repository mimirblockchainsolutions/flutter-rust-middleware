// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import 'dart:async';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class MiddleWare {

  static const MethodChannel middlewareChannel =
  const MethodChannel('mimir.labs/middleware');

   static Future<String> execute(Command cmd) async {
    String middleware;
    // Errors occurring on the platform side cause invokeMethod to throw
    // PlatformExceptions.
    try {
      final String call = await middlewareChannel.invokeMethod(
          'middleWare',
          <String, dynamic>{
            'method': cmd.method,
            'params': cmd.params,
          }
      );
      Map<String, dynamic> result = json.decode(call);
      middleware = '${result['Ok']}!';
    } on PlatformException {
      middleware = 'Failed to call ${cmd.method}.';
    }
    return middleware;
  }
}

class Command {
  Command(this.method, this.params);

  final String method;
  final dynamic params;
}


class WalletScreen extends StatefulWidget {
  @override
  _WalletScreenState createState() => new _WalletScreenState();
}

class _WalletScreenState extends State<WalletScreen> {

  String _middleware = 'Call a method.';

  Future<void> middleware() async {
     String middleware =  await MiddleWare.execute(
        new Command('hello-json', 'Woahnahnah')
      );
     setState(() {
       _middleware = middleware;
     });
  }

  @override
  Widget build(BuildContext context) {
    return new Material(
      child: new Column(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: <Widget>[
          new Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              new Text(_middleware, key: const Key('Response')),
              new Padding(
                padding: const EdgeInsets.all(16.0),
                child: new RaisedButton(
                  child: const Text('Call'),
                  onPressed: () => middleware(),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

//void main() {
//  runApp(new MaterialApp(home: new PlatformChannel()));
//}

void main() {
  runApp(MaterialApp(
    title: 'Navigation Basics',
    home: FirstScreen(),
  ));
}

class FirstScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('First Screen'),
      ),
      body: Center(
        child: RaisedButton(
          child: Text('Launch screen'),
          onPressed: () {
            Navigator.push(
              context,
              MaterialPageRoute(builder: (context) => SecondScreen()),
            );
          },
        ),
      ),
    );
  }
}

class SecondScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Second Screen"),
      ),
      body: Center(
        child: RaisedButton(
          onPressed: () {
            Navigator.push(
              context,
              MaterialPageRoute(builder: (context) => WalletScreen()),
            );
          },
          child: Text('Wallet screen'),
        ),
      ),
    );
  }
}

