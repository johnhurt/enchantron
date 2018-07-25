//
//  RustString.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/25/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class RustString {
  
  private static var binding : ext_rust_string?
  
  public class func set_binding(int_binding: ext_rust_string?) {
    RustString.binding = int_binding
  }
  
  public let rawPointer: UnsafeMutableRawPointer?
  
  public convenience init (source: String) {
    let sourceData = source.data(
        using: String.Encoding.utf8,
        allowLossyConversion: false)!
    let length = sourceData.count
    let result = RustString.binding!.new(Int64(length))
    let content = RustString.binding!.get_content(result)
    let writableContent = UnsafeMutablePointer.init(mutating: content)
    sourceData.copyBytes(to: writableContent!, count: length)
    
    self.init(rawPointer: result)
  }
  
  public init(rawPointer: UnsafeMutableRawPointer?) {
    self.rawPointer = rawPointer
  }
  
  public func toString() -> String {
    let length = (RustString.binding!.get_length)(rawPointer)
    let content = (RustString.binding!.get_content)(rawPointer)
    let data = Data(bytes: UnsafeRawPointer(content!), count: Int(length))
    
    return String(data: data, encoding: String.Encoding.utf8)!
  }
}
