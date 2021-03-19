//
//  Terrain.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 1/9/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//


#include <metal_stdlib>
#include <simd/simd.h>

#import "ShaderTypes.h"

using namespace metal;

constant float4 C = float4(0.2113248,
                           // (3.0-sqrt(3.0))/6.0
                           0.3660254,
                           // 0.5*(sqrt(3.0)-1.0)
                           -0.5773502,
                           // -1.0 + 2.0 * C.x
                           0.02439024);
                           // 1.0 / 41.0

constant bool RUN_TESTS = true;

// Some useful functions
float3 mod289(float3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
float2 mod289(float2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
float2 mod289(float4 x) { return mod289(mod289(x.xz) + mod289(x.yw)); }
float3 permute(float3 x) { return mod289(((x * 34.0) + 1.0)*x); }
//float3 permute(float3 x, float2 seed) { return mod289(((x*seed.x)+seed.y)*x); }

float2 quickTwoSum(float a,float b){
    float s=a+b;
    float e=b-(s-a);
    return float2(s,e);
}

float4 twoSumComp(float2 ari,float2 bri){
    float2 s=ari+bri;
    float2 v=s-ari;
    float2 e=(ari-(s-v))+(bri-v);
    return float4(s.x,e.x,s.y,e.y);
}

float2 two_hilo_sum(float a, float b) {
    float2 result = float2(a + b, 0.);
    result.y = b - (result.x - a);
    return result;
}

float2 two_sum(float a, float b) {
    float hi = a + b;
    float a1 = hi - b;
    float b1 = hi - a1;
    float lo = (a - a1) + (b - b1);
    return float2(hi, lo);
}

float2 df64add(float2 x, float2 y) {
    float2 hilo = two_sum(x.x, y.x);
    float2 thilo = two_sum(x.y, y.y);
    float c = hilo.y + thilo.x;
    hilo = two_hilo_sum(hilo.x, c);
    c = thilo.y + hilo.y;
    return two_hilo_sum(hilo.x, c);
}

/// Add two multi-floats together
float2 df64add2(float2 a,float2 b){
    float4 st;
    st=twoSumComp(a,b);
    st.y+=st.z;
    st.xy=quickTwoSum(st.x,st.y);
    st.y+=st.w;
    st.xy=quickTwoSum(st.x,st.y);
    return st.xy;
}

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
    float err=((aS.x*bS.x-p)
               +aS.x*bS.y+aS.y*bS.x)
    +aS.y*bS.y;
    return float2 (p,err);
}

float wtf(float a, float b, float s) {
    return fma(a, b, s);
}

float2 two_prod(float a, float b) {
    float s = a * b;
    float t = wtf(a, b, -s);
    return float2(s, t);
}

float2 df64mult(float2 x, float2 y) {
    float2 p = two_prod(x.x, y.x);
    float t = x.y * y.y;
    t = fma(x.x, y.y, t);
    t = fma(x.y, y.x, t);
    return two_hilo_sum(p.x, t + p.y);
}

float2 df64mult2(float2 a,float2 b){
    float2 p = twoProd(a.x,b.x);
    p.y+=a.x*b.y;
    p.y+=a.y*b.x;
    p=quickTwoSum(p.x,p.y);
    return p;
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

// This is like df64mult except b.y = 0
float2 mixedDf64Mult(float2 a, float b) {
    float2 p;
    p=twoProd(a.x,b);
    
    p.y += a.y * b;
    p = quickTwoSum(p.x, p.y);
    
    return p;
}

float4 getI(float4 v) {
    float2 dottedTerm = df64add(mixedDf64Mult(v.xy, C.y), mixedDf64Mult(v.zw, C.y));
    float2 ix = df64add(v.xy, dottedTerm);
    float2 iy = df64add(v.zw, dottedTerm);
    return floor(float4(ix, iy));
}

float2 getX0(float4 v, float4 i) {
    float2 dottedTerm = df64add(
            mixedDf64Mult(i.xy, C.x),
            mixedDf64Mult(i.zw, C.x));
    float2 x0x = df64add(df64add(v.xy, -i.xy), dottedTerm);
    float2 x0y = df64add(df64add(v.zw, -i.zw), dottedTerm);
    return float2(x0x.x + x0x.y, x0y.x + x0y.y);
}

//
// Description : GLSL 2D simplex noise function
//      Author : Ian McEwan, Ashima Arts
//  Maintainer : ijm
//     Lastmod : 20110822 (ijm)
//     License :
//  Copyright (C) 2011 Ashima Arts. All rights reserved.
//  Distributed under the MIT License. See LICENSE file.
//  https://github.com/ashima/webgl-noise
//
float snoise(float4 v) {
    
    // First corner (x0)
    float4 i4 = getI(v);
    float2 x0 = getX0(v, i4);
    
    // Other two corners (x1, x2)
    float2 i1 = float2(x0.x > x0.y, x0.x <= x0.y);
    float2 x1 = x0.xy + C.xx - i1;
    float2 x2 = x0.xy + C.zz;
    
    // Do some permutations to avoid
    // truncation effects in permutation
    float2 i = mod289(i4);
    float3 p = permute(
                       permute( i.y + float3(0.0, i1.y, 1.0))
                       + i.x + float3(0.0, i1.x, 1.0 ));
    
    float3 m = max(0.5 - float3(
                                dot(x0,x0),
                                dot(x1,x1),
                                dot(x2,x2)
                                ), 0.0);
    
    m = m*m ;
    m = m*m ;
    
    // Gradients:
    //  41 pts uniformly over a line, mapped onto a diamond
    //  The ring size 17*17 = 289 is close to a multiple
    //      of 41 (41*7 = 287)
    
    float3 x = 2.0 * fract(p * C.www) - 1.0;
    float3 h = abs(x) - 0.5;
    float3 ox = floor(x + 0.5);
    float3 a0 = x - ox;
    
    // Normalise gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt(a0*a0 + h*h);
    m *= 1.79284291400159 - 0.85373472095314 * (a0*a0+h*h);
    
    // Compute final noise value at P
    float3 g = float3(0.0);
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g.yz = a0.yz * float2(x1.x,x2.x) + h.yz * float2(x1.y,x2.y);
    return 130.0 * dot(m, g);
}


float4 getTerrainPoint(float2 uv, constant ViewportUniform &viewport) {
    float4 result = float4(
        viewport.topLeftMajor.x,
        viewport.topLeftMinor.x,
        viewport.topLeftMajor.y,
        viewport.topLeftMinor.y);
    
    float2 offset = uv * viewport.screenSize * viewport.scale;
    
    result.xy = mixedDf64Add(result.xy, offset.x);
    result.zw = mixedDf64Add(result.zw, offset.y);
    
    return result;
}

typedef struct
{
    float4 position [[position]];
    float2 uvCoord;
    float4 color;
} VertexOut;

bool assertEquals(float v1, float v2, float t) {
    return abs(v1 - v2) <= t;
}

bool assertEquals(float2 v1, float v2, float t) {
    return abs(v1.x + v1.y - v2) <= t;
}

bool assertEquals(float2 v1, float2 v2, float t) {
    float2 result = df64add(v1, -v2);
    return abs(dot(result, float2(1.0))) <= t;
}

bool testAddMultiFloats() {
    return assertEquals(df64add(float2(10.0, 0.1), float2(1.01, 0.001)), 11.111, 0.00001)
            && assertEquals(df64add(float2(1.2e15, 3.4e13), float2(5.6e11, 7.8e9)), 1.2345678e15, 1e9)
            && assertEquals(df64add(float2(0.03660254, 0.), float2(0.07320508, 0.)), 0.10980762, 0.0001)
            && assertEquals(floor(df64add(float2(0.2, 0.), float2(0.1098, 0.00000762))), 0.0, 0.0001);
}

bool testAddMixedFloats() {
    return assertEquals(mixedDf64Add(float2(10.0, 0.1), 1.01), 11.11, 0.0001)
            && assertEquals(mixedDf64Add(float2(1.2e15, 3.4e13), 5.6e11), 1.23456e15, 1e9);
    
}

bool testTwoProd() {
    float2 actual = two_prod(1.30987e8, 0.3660254);
    return assertEquals(actual.x, 4.794457e7, 0.0)
            && assertEquals(actual.y, -0.47050047, 0.0);
}

bool testMultiplyMultiFloats() {
    return assertEquals(df64mult(float2(123.0,0.00123), float2(4.0,0.4)), 541.2054, .001)
            && assertEquals(df64mult(float2(123.0,0.00123), float2(4.0,0.4)), 541.2054, .001);
}

bool testMultiplyMixedFloats() {
    return assertEquals(mixedDf64Mult(float2(123.0,0.00123), 4.4), 541.2054, .0001)
            && assertEquals(mixedDf64Mult(float2(0.1, 0.0), C.y), 0.03660254, 0.0001)
            && assertEquals(df64mult(
                    float2(1.30987e8, 3.56321e2), float2(C.y,0.)),
                    float2(4.79446e7, 99.4923), 0.8);
}

bool testGetI1() {
    
    float4 actual = getI(float4(13.1, 0.0356, -45.2, -0.098));
    float2 expected = floor(float2(13.1356, -45.298) + dot(float2(13.1356, -45.298), C.yy));
    return assertEquals(actual.xy, expected.x, 0.)
            && assertEquals(actual.zw, expected.y, 0.0);
}


bool testGetI2() {
    
    float4 actual = getI(float4(130987e3, 356.321, 4511e4, 1.1111e3));
    float4 expected = float4(19544e4, 3868, 10956e4, 7623);
    return assertEquals(actual.xy, expected.xy, 0.0)
            && assertEquals(actual.zw, expected.zw, 0.0);
}

bool testGetX0() {
    float2 v = float2(100.0, -500.0);
    float4 v4 = float4(v.x, 0., v.y, 0.);
    float2 i = floor(v + dot(v, C.yy));
    float4 i4 = getI(v4);
    float2 actual = getX0(v4, i4);
    float2 expected = v - i + dot(i, C.xx);
    
    return assertEquals(i4.xy, i.x, 0.)
            && assertEquals(i4.zw, i.y, 0.)
            && assertEquals(actual.x, expected.x, 0.0)
            && assertEquals(expected.y, expected.y, 0.0);
}

bool testMod289() {
    float2 actual = mod289(float4(123000, 345, -31000, -542));
    float2 expected = float2(231, -41);
    return assertEquals(actual.x, expected.x, 0)
            && assertEquals(actual.y, expected.y, 0);
}

float4 runTests() {
    
    if (!(testAddMultiFloats()
          && testAddMixedFloats()
          && testMultiplyMultiFloats()
          //&& testMultiplyMixedFloats()
          //&& testTwoProd()
        )) {
        return float4(0.5, 0.1, 0.1, 1.0);
    }
    
    if (!(testGetI1()
          && testGetI2()
          && testGetX0()
          && testMod289()
          )) {
        return float4(0.6, 0.5, 0.05, 1.0);
    }
    
    return float4(0.1,.6,.2,1.0);
}


vertex VertexOut vertexShader(uint vertexId [[vertex_id]],
                              constant ViewportUniform &viewport [[buffer(1)]])
{
    VertexOut out;
    
    float bottom = vertexId > 1;
    float right = vertexId == 1 || vertexId == 3;
    
    out.position = float4(-1.0 + 2.0 * right, 1.0 - 2.0 * bottom, 0.0, 1.0);
    out.uvCoord = float2(right, bottom);
    
    if (RUN_TESTS) {
        out.color = runTests();
    }
    
    return out;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]],
                               constant ViewportUniform &viewport [[buffer(1)]])
{
    if (!RUN_TESTS && viewport.scale < 2.) {
        return float4();
    }
    
    float4 terrainPoint = getTerrainPoint(in.uvCoord, viewport);
    float4 st = terrainPoint / 647.0;
    
    float color = snoise(st);
    //        + snoise(st * 2.0 + 2339.) / 2.0
    //        + snoise(st * 4.0 + 239.) / 4.0 ;
    
    //color *= 1.0 / (1.75);
    color = color * 0.5 + 0.5;
    
    float4 brown = float4(0.3, 0.25, 0.1, 1.0);
    float4 green = float4(0.3, 0.9, 0.6, 1.0);
    
    float4 result = color < 0.5 ? brown : green;
    
    if (RUN_TESTS) {
        result = (result + in.color) * 0.5;
    }
    
    return result;
}

