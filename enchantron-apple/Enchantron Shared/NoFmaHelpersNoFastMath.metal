//
//  NoFmaHelpers.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 6/27/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#include <metal_stdlib>
#include <simd/simd.h>

#include "NoFmaHelpers.h"

using namespace metal;

#ifdef WITHOUT_FMA

float2 split(float a){
    const float split=4097;//(1<<12)+1;
    float t=a*split;
    float ahi=t-(t-a);
    float alo=a-ahi;
    return float2(ahi,alo);
}

float2 twoProd(float a,float b){
    float p=a*b;
    float2 aS=split(a);
    float2 bS=split(b);
    float err=( ( aS.x * bS.x - p) + aS.x * bS.y + aS.y * bS.x ) + aS.y * bS.y;
    return float2 (p,err);
}

#endif

#ifdef WITH_FMA

float2 twoProd(float a, float b) {
    float s = a * b;
    float t = wtf(a, b, -s);
    return float2(s, t);
}

#endif

float2 splitUgh(float a) {
    const float split=4097;//(1<<12)+1;
    float t=a*split;
    return float2(t, t - a);
}

float2 split(float a, float2 ugh) {
    float ahi = ugh.x - ugh.y;
    float alo=a-ahi;
    return float2(ahi,alo);
}

float4 twoProd1(float a, float b, float2 uA, float2 uB){
    float2 aS=split(a, uA);
    float2 bS=split(b, uB);
    return float4(aS.x, aS.y, bS.x, bS.y);
}

float4 twoProd2(float p, float4 part1) {
    return float4((part1.x * part1.z) - p, part1.x * part1.w, part1.y * part1.z, part1.y * part1.w);
}

float2 twoProdErr(float4 part2) {
    return float2(part2.x + part2.y + part2.z, + part2.w);
}
