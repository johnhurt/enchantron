//
//  FmaHelpers.metal
//  Enchantron macOS
//
//  Created by Kevin Guthrie on 6/27/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#include <metal_stdlib>
#include "DoubleMathHelpers.h"

using namespace metal;

float2 quickTwoSum(float a,float b){
    float s=a+b;
    float e=b-(s-a);
    return float2(s,e);
}

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
    float t = fma(a, b, -s);
    return float2(s, t);
}

#endif

float2 mixedAddSt(float a, float b) {
    
    float s=a + b;
    float v=s-a;
    float e=(a-(s-v))+(b-v);
    
    return float2(s, e);
}

float2 twoSum(float a, float b) {
    float hi = a + b;
    float a1 = hi - b;
    float b1 = hi - a1;
    float lo = (a - a1) + (b - b1);
    return float2(hi, lo);
}
