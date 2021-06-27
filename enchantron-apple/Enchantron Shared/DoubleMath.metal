//
//  DoubleMath.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 6/23/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#include <simd/simd.h>
#include "DoubleMath.h"

using namespace metal;

float2 quickTwoSum(float a,float b){
    float s=a+b;
    float e=b-(s-a);
    return float2(s,e);
}

// This is like df64Add but with b.y = 0
float2 mixedDf64Add(float2 a, float b) {
    float2 st;
    
    float s=a.x + b;
    float v=s-a.x;
    float e=(a.x-(s-v))+(b-v);
    
    st = float2(s, e);
    
    st.y+=a.y;
    st.xy=quickTwoSum(st.x,st.y);
    st.xy=quickTwoSum(st.x,st.y);
    return st.xy;
}

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


/* ************* Functions that don't use fma ****************** */

#ifdef WITHOUT_FMA

float4 twoProd1(float a, float b){
    float2 aS=split(a, splitUgh(a));
    float2 bS=split(b, splitUgh(b));
    return float4(aS, bS);
}

float4 twoProd2(float p, float highs, float4 part1) {
    return float4(p - highs, part1.x * part1.w, part1.y * part1.z, + part1.y * part1.w);
}

float2 twoProd(float a,float b) {
    float p = a * b;
    float4 part1 = twoProd1(a, b);
    float highs = part1.x * part1.z;
    float4 part2 = twoProd2(p, highs, part1);
    return float2(p, part2.x + part2.y + part2.z + part2.w);
}

float2 df64Mult(float2 a,float2 b){
    float2 p = twoProd(a.x,b.x);
    p.y+=a.x*b.y;
    p.y+=a.y*b.x;
    p=quickTwoSum(p.x,p.y);
    return p;
}

#endif

/* ************** Functions that do use fma ****************** */

#ifdef WITH_FMA

//float2 twoProd(float a, float b) {
//    float s = a * b;
//    float t = fma(a, b, -s);
//    return float2(s, t);
//}

float2 df64Mult(float2 x, float2 y) {
    float2 p = twoProd(x.x, y.x);
    float t = x.y * y.y;
    t = fma(x.x, y.y, t);
    t = fma(x.y, y.x, t);
    return quickTwoSum(p.x, t + p.y);
}

#endif

/* ************* Functions that don't need fma ************** */

float4 twoSumComp(float2 ari,float2 bri){
    float2 s=ari+bri;
    float2 v=s-ari;
    float2 e=(ari-(s-v))+(bri-v);
    return float4(s.x,e.x,s.y,e.y);
}

float2 two_sum(float a, float b) {
    float hi = a + b;
    float a1 = hi - b;
    float b1 = hi - a1;
    float lo = (a - a1) + (b - b1);
    return float2(hi, lo);
}

float2 df64Add(float2 x, float2 y) {
    float2 hilo = two_sum(x.x, y.x);
    float2 thilo = two_sum(x.y, y.y);
    float c = hilo.y + thilo.x;
    hilo = quickTwoSum(hilo.x, c);
    c = thilo.y + hilo.y;
    return quickTwoSum(hilo.x, c);
}

// This is like df64Mult except b.y = 0
float2 mixedDf64Mult(float2 a, float b) {
    float2 p;
    p=twoProd(a.x,b);
    
    p.y += a.y * b;
    p = quickTwoSum(p.x, p.y);
    
    return p;
}

/// Add two multi-floats together
//float2 df64Add2(float2 a,float2 b){
//    float4 st;
//    st=twoSumComp(a,b);
//    st.y+=st.z;
//    st.xy=quickTwoSum(st.x,st.y);
//    st.y+=st.w;
//    st.xy=quickTwoSum(st.x,st.y);
//    return st.xy;
//}

