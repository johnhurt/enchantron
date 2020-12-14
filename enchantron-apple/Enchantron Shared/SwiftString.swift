//
//  SwiftString.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/26/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class SwiftString {
    
    let length: Int64
    let data: NSData
    
    init(_ source: String) {
        let sourceData = source.data(
            using: String.Encoding.utf8,
            allowLossyConversion: false)!
        self.length = Int64(sourceData.count)
        self.data = sourceData as NSData
    }
    
    func getContent() -> UnsafeMutablePointer<UInt8> {
        return UnsafeMutablePointer.init(mutating: self.data.bytes.assumingMemoryBound(to: UInt8.self))
    }
    
    func getLength() -> Int64 {
        return self.length
    }
    
    
    deinit {
        print("Dropping Swift String")
    }
}

private func getLength(ref: UnsafeMutableRawPointer?) -> Int64 {
    let _self : SwiftString = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
    return _self.length
}

private func getContent(ref: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<UInt8>? {
    let _self : SwiftString = Unmanaged.fromOpaque(ref!).takeUnretainedValue()
    return UnsafeMutablePointer.init(mutating: _self.data.bytes.assumingMemoryBound(to: UInt8.self))
}

private func destroy(ref: UnsafeMutableRawPointer?) -> Void {
    let _ : SwiftString = Unmanaged.fromOpaque(ref!).takeRetainedValue()
}
