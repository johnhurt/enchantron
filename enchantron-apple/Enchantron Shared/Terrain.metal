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
#import "DoubleMath.h"

#define RUN_TESTS

using namespace metal;

constant float4 C = float4(0.2113248,
                           // (3.0-sqrt(3.0))/6.0
                           0.3660254,
                           // 0.5*(sqrt(3.0)-1.0)
                           -0.5773502,
                           // -1.0 + 2.0 * C.x
                           0.02439024);
                           // 1.0 / 41.0

// Some useful functions
float mod289(float x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
float2 mod289(float2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
float3 mod289(float3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }

float multiDfMod289(float2 x) {
    float2 rawMod = mod289(x);
    float useSmall = abs(x.y) > 0;
    
    float result = rawMod.x + useSmall * rawMod.y;
    
    return result - ((result >= 289) * 289);
}

float3 permute(float3 x) { return mod289(((x * 34.0) + 1.0)*x); }
//float3 permute(float3 x, float2 seed) { return mod289(((x*seed.x)+seed.y)*x); }

float2 getIStep1(float4 v) {
    return mixedDf64Mult(v.xy, C.y);
}

float2 floorDfMulti(float2 v) {
    float2 rawFloor = floor(v);
    float x = abs(trunc(v.y)) > 1;
    float y = v.y < 0;
    float2 diff = float2(!x * y, x * y);
    return rawFloor + diff;
}

float4 getI(float4 v) {
    float2 dottedTerm = df64Add(mixedDf64Mult(v.xy, C.y), mixedDf64Mult(v.zw, C.y));
    float2 ix = df64Add(v.xy, dottedTerm);
    float2 iy = df64Add(v.zw, dottedTerm);
    return float4(floorDfMulti(ix), floorDfMulti(iy));
}

float2 getX0(float4 v, float4 i) {
    float2 dottedTerm = df64Add(
            mixedDf64Mult(i.xy, C.x),
            mixedDf64Mult(i.zw, C.x));
    float2 x0x = df64Add(df64Add(v.xy, -i.xy), dottedTerm);
    float2 x0y = df64Add(df64Add(v.zw, -i.zw), dottedTerm);
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
    float2 i = float2(multiDfMod289(i4.xy), multiDfMod289(i4.zw));
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

float4 runTests();

vertex VertexOut vertexShader(uint vertexId [[vertex_id]],
                              constant ViewportUniform &viewport [[buffer(1)]])
{
    VertexOut out;
    
    float bottom = vertexId > 1;
    float right = vertexId == 1 || vertexId == 3;
    
    out.position = float4(-1.0 + 2.0 * right, 1.0 - 2.0 * bottom, 0.0, 1.0);
    out.uvCoord = float2(right, bottom);
    
#ifdef RUN_TESTS
    out.color = runTests();
#endif
    
    return out;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]],
                               constant ViewportUniform &viewport [[buffer(1)]])
{
#ifndef RUN_TESTS
    if (viewport.scale < 2.) {
        return float4();
    }
#endif
    
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
    
#ifdef RUN_TESTS
        result = (result + in.color) * 0.5;
#endif
    
    return result;
}

/* Test */

#ifdef RUN_TESTS

bool assertEquals(float v1, float v2, float t) {
    return abs(v1 - v2) <= t;
}

bool assertEquals(float2 v1, float v2, float t) {
    return abs(v1.x + v1.y - v2) <= t;
}

bool assertEquals(float2 v1, float2 v2, float t) {
    float2 result = df64Add(v1, -v2);
    return abs(dot(result, float2(1.0))) <= t;
}

bool testAddMultiFloats() {
    return true
        && assertEquals(df64Add(float2(10.0, 0.1), float2(1.01, 0.001)), float2(11.111, -6.9034286e-8), 0)
        && assertEquals(df64Add(float2(1.2e15, 3.4e13), float2(5.6e11, 7.8e9)), float2(1.2345678e15, 2.3760384e7), 0)
        && assertEquals(df64Add(float2(0.03660254, 0.), float2(0.07320508, 0.)), 0.109807625, 0)
        && assertEquals(df64Add(float2(0.2, 0.), float2(0.1098, 0.00000762)), float2(0.30980763, -1.944045e-9), 0)
    ;
}

bool testAddMixedFloats() {
    return assertEquals(mixedDf64Add(float2(10.0, 0.1), 1.01), 11.11, 0.0001)
            && assertEquals(mixedDf64Add(float2(1.2e15, 3.4e13), 5.6e11), 1.23456e15, 1e9);
    
}

bool testBasicMath() {
    return
        assertEquals(4097. * 1.30987e8, 536653730000.0, 0)
        && assertEquals(536653730000.0 - (536653730000.0 - 1.30987e8), 130973700.0, 0)
        && assertEquals(1.30987e8 - 130973700.0, 13304.0, 0)
    ;
}

#ifdef WITHOUT_FMA

#include "NoFmaHelpers.h"

bool testSplit() {
    float2 actual1 = split(1.30987e8, splitUgh(1.30987e8));
    float2 actual2 = split(0.3660254, splitUgh(0.3660254));
    return assertEquals(actual1.x, 130973700.0, 0.0)
            && assertEquals(actual1.y, 13304.0, 0)
            && assertEquals(actual2.x, 0.3659668, 0.0)
            && assertEquals(actual2.y, 0.000058591366, 0)
    ;
}

bool testTwoProd1() {
    float4 actual = twoProd1(1.30987e8, 0.3660254, float2(536653730000.0, 536522750000.0), float2(1499.606, 1499.24));
    
    return true
        && assertEquals(actual.x, 130973700.0, 0.0)
        && assertEquals(actual.y, 13304.0, 0)
        && assertEquals(actual.z, 0.3659668, 0.0)
        && assertEquals(actual.w, 0.000058591366, 0)
    ;
}

bool testTwoProd2() {
    float4 part1 = float4(130973700.0, 13304.0, 0.3659668, 0.000058591366);
    float p0 = 47944570.0;
    
    float4 actual = twoProd2(p0, part1);
    return true
        && assertEquals(actual.x, -12544.0, 0)
        && assertEquals(actual.y, 7673.9277, 0)
        && assertEquals(actual.z, 4868.8223, 0)
        && assertEquals(actual.w, 0.77949953, 0)
    ;
}

bool testTwoProdErr() {
    float2 actual = twoProdErr(float4(-12544.0, 7673.9277, 4868.8223, 0.77949953));
    return assertEquals(actual.x, -1.25, 0.0)
            && assertEquals(actual.y, 0.77949953, 0.0)
    ;
}

#endif

bool testTwoProd() {
    float2 actual = twoProd(1.30987e8, 0.3660254);
    return true
        && assertEquals(actual.x, 47944570.0, 0)
        && assertEquals(actual.y, -0.47050047, 0)
    ;
}

bool testTwoProdAgain() {
    float2 actual = twoProd(130987e3, 0.3660254);
    return true
        && assertEquals(actual.x, 47944570.0, 0)
        && assertEquals(actual.y, -0.47050047, 0)
    ;
}

bool testMultiplyMultiFloats() {
    return assertEquals(df64Mult(float2(123.0,0.00123), float2(4.0,0.4)), 541.2054, .001)
            && assertEquals(df64Mult(float2(123.0,0.00123), float2(4.0,0.4)), 541.2054, .001);
}

bool testMultiplyMixedFloats() {
    float2 actual1 = mixedDf64Mult(float2(123.0,0.00123), 4.4);
    float2 actual2 = mixedDf64Mult(float2(0.1, 0.0), C.y);
    
    return true
        && assertEquals(actual1.x, 541.20544, 0)
        && assertEquals(actual1.y, -2.0605512e-5, 0)
        && assertEquals(actual2.x, 0.036602538, 0)
        && assertEquals(actual2.y, 1.2904784e-9, 0)
    ;
        
    
//    return assertEquals(mixedDf64Mult(float2(123.0,0.00123), 4.4), 541.2054, .0001)
//            && assertEquals(mixedDf64Mult(float2(0.1, 0.0), C.y), 0.03660254, 0.0001)
//            && assertEquals(df64Mult(
//                    float2(1.30987e8, 3.56321e2), float2(C.y,0.)),
//                    float2(4.79446e7, 99.4923), 0.8);
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
    return true
            && assertEquals(actual.xy, expected.xy, 0.0)
            && assertEquals(actual.zw, expected.zw, 0.0)
    ;
}


bool testGetIStep1() {
    float2 actual = getIStep1(float4(130987e3, 356.321, 4511e4, 1.1111e3));
    float2 expected = float2(4.7944696e7, 1.9520264);
    
    return true
            && assertEquals(actual.x, expected.x, 0.0)
            && assertEquals(actual.y, expected.y, 0.0)
    ;
    
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
    float actual1 = multiDfMod289(float2(123000, 345));
    float actual2 = multiDfMod289(float2(-31000, -542));
    float2 expected = float2(231, 289-41);
    return true
        && assertEquals(actual1, expected.x, 0)
        && assertEquals(actual2, expected.y, 0)
    ;
}

float4 runTests() {
    
    if (!(testAddMultiFloats()
          && testAddMixedFloats()
          && testMultiplyMultiFloats()
          && testMultiplyMixedFloats()
          && testBasicMath()
          
          && testTwoProd()
          
        )) {
        return float4(0.5, 0.1, 0.1, 1.0);
    }
    
    if (!(testGetI1()
         && testGetIStep1()
         && testGetI2()
         //&& testGetX0()
         //&& testMod289()
          )) {
        return float4(0.6, 0.5, 0.05, 1.0);
    }
    
    return float4(0.1,.6,.2,1.0);
}


#endif
