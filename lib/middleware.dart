import 'dart:async';

import 'package:flutter/services.dart';

class Middleware {
  static const MethodChannel _channel =
      const MethodChannel('middleware');

  static Future<String> get platformVersion async {
    final String version = await _channel.invokeMethod('getPlatformVersion');
    return version;
  }
}
