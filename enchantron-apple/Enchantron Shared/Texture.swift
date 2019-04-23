//
//  Texture.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/20/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit
//
//class Texture {
//  
//  public class func get_binding() -> ext_texture {
//    return ext_texture(
//      get_sub_texture: get_sub_texture,
//      get_width: get_width,
//      get_height: get_height,
//      destroy: destroy)
//  }
//  
//  let texture : SKTexture
//  
//  public init(texture: SKTexture) {
//    self.texture = texture
//  }
//  
//  public convenience init(resourceName: String) {
//    self.init(texture: SKTexture(imageNamed: "Resources/Textures/" + resourceName))
//  }
//
//  public func getSubTexture(left: Int64, top: Int64, width: Int64, height: Int64) -> Texture {
//    let size = CGSize(width: Int(width), height: Int(height))
//    let rect = CGRect(origin: CGPoint(x: Int(top), y: Int(left)), size: size)
//    return Texture(texture: SKTexture(rect: rect, in: self.texture))
//  }
//  
//  deinit {
//    print("Dropping Texture")
//  }
//}
//
//private func get_sub_texture(ref: UnsafeMutableRawPointer?, left: Int64, top: Int64, width: Int64, height: Int64)
//    -> UnsafeMutableRawPointer? {
//  let texture : Texture = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
//  let result = texture.getSubTexture(left: left, top: top, width: width, height: height)
//  return UnsafeMutableRawPointer(Unmanaged.passRetained(result).toOpaque())
//}
//
//private func get_width(ref: UnsafeMutableRawPointer?) -> Int64 {
//  let texture : Texture = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
//  return Int64(texture.texture.size().width)
//}
//
//private func get_height(ref: UnsafeMutableRawPointer?) -> Int64 {
//  let texture : Texture = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
//  return Int64(texture.texture.size().height)
//}
//
//private func destroy(ref: UnsafeMutableRawPointer?) -> Void {
//  let _ : Texture = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
//}
