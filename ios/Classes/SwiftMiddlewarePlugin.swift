import Flutter
import UIKit

public class SwiftMiddlewarePlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    let channel = FlutterMethodChannel(name: "middleware", binaryMessenger: registrar.messenger())
    let instance = SwiftMiddlewarePlugin()
    registrar.addMethodCallDelegate(instance, channel: channel)
  }

  private func middleWare(args: String, result: FlutterResult) {
    let middleWare = MiddleWare();
    let reslt = middleWare.call(to: args);
    result(reslt);
  }

  public func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
    if ("middleWare" == call.method) {
        let jsonResult = call.arguments as! NSArray;
        let array = jsonResult as? [Any];
        let argsObject = array?.first;
        let aaargs = argsObject as? [String: Any];
        let jsonData = try! JSONSerialization.data(withJSONObject: aaargs!, options: .prettyPrinted);
        let jsonString = NSString(data: jsonData, encoding: String.Encoding.utf8.rawValue)! as String;
        print(jsonString);
        
        self.middleWare(args: jsonString, result: result);
    } else {
        result(FlutterMethodNotImplemented);
    };
    
    result("iOS " + UIDevice.current.systemVersion)
  }
}
