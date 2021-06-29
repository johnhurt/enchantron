//
//  YesFmaHelpers.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 6/27/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#include <metal_stdlib>
#include <simd/simd.h>
#include "YesFmaHelpers.h"

using namespace metal;

float2 twoProdHelper(float a, float b, float s) {
    float t = fma(a, b, -s);
    return float2(s, t);
}

