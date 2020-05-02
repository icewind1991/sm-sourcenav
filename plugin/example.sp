#include <sourcenav>

new SourceNav:sourceNav = null;

public void OnPluginStart() {
    new String:map[128];
    GetCurrentMap(map, sizeof(map));

    new String:nav[128];
    Format(nav, sizeof(nav), "tf/maps/%s.nav", map);
    PrintToServer("Loading %s", nav);
    sourceNav = new SourceNav(nav);

    RegServerCmd("sm_nav_query", Query, "Query the nav file at x/y point");
}

public Action:Query(args) {
    new String:x_str[128];
    GetCmdArg(1, x_str, sizeof(x_str));
    new String:y_str[128];
    GetCmdArg(2, y_str, sizeof(y_str));
    float x = StringToFloat(x_str);
    float y = StringToFloat(y_str);

    PrintToServer("x: %f, y: %f", x, y);

    float z = sourceNav.query(x, y, 0.0);

    PrintToServer("Z: %f", z);
    return Plugin_Handled;
}