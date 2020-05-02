#if defined _sourcenav_included
 #endinput
#endif
#define _sourcenav_included

methodmap SourceNav < Handle {
    public native SourceNav(char[] path);
    public native float query(float x, float y, float z_guess);
}

public Extension __ext_sourcenav =
{
	name = "sourcenav",
	file = "sourcenav.ext",
	autoload = 1,

#if defined REQUIRE_EXTENSIONS
	required = 1,
#else
	required = 0,
#endif
};

#if !defined REQUIRE_EXTENSIONS
public void __ext_sourcenav_SetNTVOptional()
{

}
#endif