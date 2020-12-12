//
//  ShaderTypes.h
//  Enchantron
//
//  Created by Kevin Guthrie on 11/30/20.
//  Copyright © 2020 Rook And Pawn Industries, Inc. All rights reserved.
//

#ifndef ShaderTypes_h
#define ShaderTypes_h

#ifdef __METAL_VERSION__
#define NS_ENUM(_type, _name) enum _name : _type _name; enum _name : _type
#define NSInteger metal::int32_t
#else
#import <Foundation/Foundation.h>
#endif

#include <simd/simd.h>

typedef struct
{
    vector_float2 topLeftMajor;
    vector_float2 topLeftMinor;
    vector_float2 screenSize;
    float scale;
} ViewportUniform;

typedef struct {
    vector_float2 topLeftMajor;
    vector_float2 topLeftMinor;
    vector_float2 size;
    uint color;
    vector_float2 textureUvTopLeft;
    vector_float2 textureUvSize;
} SpriteUniform;

#endif /* ShaderTypes_h */
