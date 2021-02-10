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

constant float MAJOR_SIZE = 512.;
constant float4 C = float4(0.2113248,
                           // (3.0-sqrt(3.0))/6.0
                           0.3660254,
                           // 0.5*(sqrt(3.0)-1.0)
                           -0.5773502,
                           // -1.0 + 2.0 * C.x
                           0.02439024);
// 1.0 / 41.0

constant float CyNum = 1830127.;
constant float CyDenom = 5000000.;

// Some useful functions
float3 mod289(float3 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
float2 mod289(float2 x) { return x - floor(x * (1.0 / 289.0)) * 289.0; }
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

float2 df64add(float2 a,float2 b){
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

float2 df64mult(float2 a,float2 b){
    float2 p;
    p=twoProd(a.x,b.x);
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
    float2 ix = floor(v.xy + (v.x + v.y) * C.y);
    float2 iMin = floor(v.zw + (v.z + v.w) * C.y)
    return float4(iMaj, iMin);
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
    float4 i = getI(v);
    float4 x0 = v - i + dot(i, C.xxxx);
    
    // Other two corners (x1, x2)
    float2 i1 = float2(0.0);
    i1 = (x0.x > x0.y)? float2(1.0, 0.0):float2(0.0, 1.0);
    float2 x1 = x0.xy + C.xx - i1;
    float2 x2 = x0.xy + C.zz;
    
    // Do some permutations to avoid
    // truncation effects in permutation
    i = mod289(i);
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
        viewport.topLeftMajor.x * 512.0,
        viewport.topLeftMinor.x,
        viewport.topLeftMajor.y * 512.0,
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
} VertexOut;

vertex VertexOut vertexShader(uint vertexId [[vertex_id]],
                              constant ViewportUniform &viewport [[buffer(1)]])
{
    VertexOut out;
    
    float bottom = vertexId > 1;
    float right = vertexId == 1 || vertexId == 3;
    
    out.position = float4(-1.0 + 2.0 * right, 1.0 - 2.0 * bottom, 0.0, 1.0);
    out.uvCoord = float2(right, bottom);
    
    return out;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]],
                               constant ViewportUniform &viewport [[buffer(1)]])
{
    if (viewport.scale < 2.) {
        return float4();
    }
    
    float2 terrainPoint = getTerrainPoint(in.uvCoord, viewport);
    float2 st = terrainPoint / 647.0;
    
    float color = snoise(st);
    //        + snoise(st * 2.0 + 2339.) / 2.0
    //        + snoise(st * 4.0 + 239.) / 4.0 ;
    
    //color *= 1.0 / (1.75);
    color = color * 0.5 + 0.5;
    
    float4 brown = float4(0.3, 0.25, 0.1, 1.0);
    float4 green = float4(0.3, 0.9, 0.6, 1.0);
    
    return color < 0.5 ? brown : green;
}

