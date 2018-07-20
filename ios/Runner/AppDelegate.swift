// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

import UIKit
import Flutter

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {

  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {
    GeneratedPluginRegistrant.register(with: self);
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController;
    let middlewareChannel = FlutterMethodChannel.init(name: "mimir.labs/middleware",
                                                   binaryMessenger: controller);
    middlewareChannel.setMethodCallHandler({
      (call: FlutterMethodCall, result: FlutterResult) -> Void in
      if ("middleWare" == call.method) {
        self.middleWare(result: result);
      } else {
        result(FlutterMethodNotImplemented);
      }
    });

    return super.application(application, didFinishLaunchingWithOptions: launchOptions);
  }

  private func middleWare(result: FlutterResult) {
    let middleWare = MiddleWare();
    let reslt = middleWare.sayHello(to: "from Rust!");
    print(reslt);
    result(reslt);
  }

}
