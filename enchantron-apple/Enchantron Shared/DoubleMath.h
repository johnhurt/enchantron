//
//  DoubleMath.h
//  Enchantron
//
//  Created by Kevin Guthrie on 6/23/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#ifndef DoubleMath_h
#define DoubleMath_h

#include <simd/simd.h>

vector_float2 twoProd(float a, float b);

vector_float2 df64Mult(vector_float2 x, vector_float2 y);

vector_float2 df64Add(vector_float2 x, vector_float2 y);

vector_float2 mixedDf64Mult(vector_float2 a, float b);

vector_float2 mixedDf64Add(vector_float2 a, float b);

#endif /* DoubleMath_h */
