//
//  Future.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 12/29/19.
//  Copyright © 2019 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class RustFuture<T> {
    
    private let ref: OpaquePointer?
    
    init(_ ref: OpaquePointer?) {
        self.ref = ref
    }
    
}
