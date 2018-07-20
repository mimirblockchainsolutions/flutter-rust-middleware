package labs.mimir.middleware;

import android.os.Bundle;

import io.flutter.app.FlutterActivity;
import io.flutter.plugin.common.MethodChannel;
import io.flutter.plugin.common.MethodChannel.MethodCallHandler;
import io.flutter.plugin.common.MethodChannel.Result;
import io.flutter.plugin.common.MethodCall;
import io.flutter.plugins.GeneratedPluginRegistrant;

public class MainActivity extends FlutterActivity {

  static {
    System.loadLibrary("middleware");
  }

  private static final String MIDDLEWARE_CHANNEL = "mimir.labs/middleware";

  @Override
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    GeneratedPluginRegistrant.registerWith(this);

    new MethodChannel(getFlutterView(), MIDDLEWARE_CHANNEL).setMethodCallHandler(
            new MethodCallHandler() {
              @Override
              public void onMethodCall(MethodCall call, Result result) {
                if (call.method.equals("middleWare")) {
                  String res = middleWare();

                  if (res.length() > 0) {
                    result.success(res);
                  } else {
                    result.error("UNAVAILABLE", "Method unavailable.", null);
                  }
                } else {
                  result.notImplemented();
                }
              }
            }
    );
  }

  private String middleWare() {
    MiddleWare m = new MiddleWare();
    return m.call("from Rust!");
  }
}
