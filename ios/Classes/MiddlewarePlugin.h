#import <Flutter/Flutter.h>
#include <stdint.h>

const char* request_function(const char* to);
void function_free(char *);

@interface MiddlewarePlugin : NSObject<FlutterPlugin>
@end
