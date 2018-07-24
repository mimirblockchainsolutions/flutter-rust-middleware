package labs.mimir.middleware;

import org.json.JSONObject;

public class MiddleWare {

    private static native String result(final String pattern);

    public String call(String to) {
        return result(to);
    }
}
