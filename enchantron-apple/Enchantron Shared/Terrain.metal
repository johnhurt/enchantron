//
//  Terrain.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 1/9/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//


#include <metal_stdlib>
#include <simd/simd.h>

// Including header shared between this Metal shader code and Swift/C code executing Metal API commands
#import "ShaderTypes.h"

using namespace metal;



typedef struct
{
    float4 position [[position]];
} VertexOut;

vertex VertexOut vertexShader(uint vertexId [[vertex_id]],
                              constant ViewportUniform &viewport [[buffer(1)]])
{
    VertexOut out;
    
    float bottom = vertexId > 1;
    float right = vertexId == 1 || vertexId == 3;
    
    out.position = float4(-1.0 + 2.0 * right, 1.0 - 2.0 * bottom, 0.0, 1.0);
    
    return out;
}

fragment float4 fragmentShader(VertexOut in [[stage_in]],
                               texture2d<float> tex     [[ texture(0) ]])
{
    return float4(float4(0.2, 0.3, 0.5, 1.0));
}

