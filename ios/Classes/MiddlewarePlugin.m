#import "MiddlewarePlugin.h"
#import <middleware/middleware-Swift.h>

@implementation MiddlewarePlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftMiddlewarePlugin registerWithRegistrar:registrar];
}
@end
