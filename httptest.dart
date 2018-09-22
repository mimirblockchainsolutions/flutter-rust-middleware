import 'package:http/http.dart' as http;

void main() {
  var url = "http://google.com/";
  http.post(url, body: {""})
      .then((response) {
    print("Response status: ${response.statusCode}");
  });}
