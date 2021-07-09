//
//  SpriteShaders.metal
//  Enchantron
//
//  Created by Kevin Guthrie on 12/10/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//


#include <metal_stdlib>
#include <simd/simd.h>

// Including header shared between this Metal shader code and Swift/C code executing Metal API commands
#import "ShaderTypes.h"

using namespace metal;

typedef struct
{
    float4 position [[position]];
    float2 texCoord;
    bool hasTexture;
    float4 color;
} VertexOut;

vertex VertexOut spriteVertexShader(uint vertexId [[vertex_id]],
                              constant SpriteUniform &uniforms [[buffer(0)]],
                              constant ViewportUniform &viewport [[buffer(1)]])
{
    VertexOut out;
    
    out.color = uniforms.color;
    out.hasTexture = uniforms.hasTexture;
    
    // Get the viewport size and cast to float.
    float2 halfViewportSize = float2(viewport.screenSize * viewport.scale) / 2;
    float2 halfVpSizeFlipY = float2(1, -1) * halfViewportSize;
    
    float bottom = vertexId > 1;
    float right = vertexId == 1 || vertexId == 3;
    
    float2 shiftedTopLeft = (uniforms.topLeftMajor - viewport.topLeftMajor)
         + (uniforms.topLeftMinor - viewport.topLeftMinor);
    
    out.position = vector_float4(
             shiftedTopLeft.x + right * uniforms.size.x,
             shiftedTopLeft.y + bottom * uniforms.size.y,
             0,
             1.0);
    out.position.xy = (out.position.xy - halfViewportSize) / halfVpSizeFlipY;

    out.texCoord = vector_float2(
             uniforms.textureUvTopLeft.x + right * uniforms.textureUvSize.x,
             uniforms.textureUvTopLeft.y + bottom * uniforms.textureUvSize.y);

    out.color.a = min(max((4 - viewport.scale * 16) / 2, (float)!out.hasTexture), 1.0) ;
    
    return out;
}

fragment float4 spriteFragmentShader(VertexOut in [[stage_in]],
                               texture2d<float> tex     [[ texture(0) ]])
{
    constexpr sampler defaultSampler;
    
    float4 colorSample;
    
    if (in.hasTexture) {
        colorSample = tex.sample(defaultSampler, in.texCoord.xy);
        colorSample.a = min(in.color.a, colorSample.a);
    }
    else {
        colorSample = in.color;
    }
    
    return colorSample;
}
