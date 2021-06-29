//
//  NoFmaHelpers.h
//  Enchantron
//
//  Created by Kevin Guthrie on 6/27/21.
//  Copyright © 2021 Rook And Pawn Industries, Inc. All rights reserved.
//

#ifndef NoFmaHelpers_h
#define NoFmaHelpers_h

vector_float2 splitUgh(float a);

vector_float2 split(float a, vector_float2 ugh);

vector_float4 twoProd1(float a, float b, vector_float2 uA, vector_float2 uB);

vector_float4 twoProd2(float v0, vector_float4 c);

vector_float2 twoProdErr(vector_float4 part2);


#endif /* NoFmaHelpers_h */
