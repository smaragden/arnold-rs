#include <ai.h>

AtBBox AiBBoxUnion_NOINLINE(const AtBBox &b1, const AtBBox &b2)
{
    return AiBBoxUnion(b1, b2);
}

AtBBox AiBBoxIntersection_NOINLINE(const AtBBox &b1, const AtBBox &b2)
{
    return AiBBoxIntersection(b1, b2);
}

AtBBox AiBBoxLerp_NOINLINE(float k, const AtBBox &lo, const AtBBox &hi)
{
    return AiBBoxLerp(k, lo, hi);
}
