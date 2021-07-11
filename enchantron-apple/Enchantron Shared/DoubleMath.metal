//
//  DoubleMath.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 6/23/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#include <simd/simd.h>
#include "DoubleMathHelpers.h"
#include "DoubleMath.h"

using namespace metal;


/* ************* Functions that don't use fma ****************** */

#ifdef WITHOUT_FMA

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

float2 df64Mult(float2 x, float2 y) {
    float2 p = twoProd(x.x, y.x);
    float t = x.y * y.y;
    t = fma(x.x, y.y, t);
    t = fma(x.y, y.x, t);
    return quickTwoSum(p.x, t + p.y);
}

#endif

/* ************* Functions that don't need fma ************** */

// This is like df64Mult except b.y = 0
float2 mixedDf64Mult(float2 a, float b) {
    float2 p = twoProd(a.x,b);
    
    p.y += a.y * b;
    p = quickTwoSum(p.x, p.y);
    
    return p;
}

// This is like df64Add but with b.y = 0
float2 mixedDf64Add(float2 a, float b) {
    float2 st = mixedAddSt(a.x, b);
    st.y+=a.y;
    st.xy=quickTwoSum(st.x,st.y);
    st.xy=quickTwoSum(st.x,st.y);
    return st.xy;
}

float2 df64Add(float2 x, float2 y) {
    float2 hilo = twoSum(x.x, y.x);
    float2 thilo = twoSum(x.y, y.y);
    float c = hilo.y + thilo.x;
    hilo = quickTwoSum(hilo.x, c);
    c = thilo.y + hilo.y;
    return quickTwoSum(hilo.x, c);
}
