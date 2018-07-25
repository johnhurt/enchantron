//
//  TextureLoader.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class TextureLoader {
  
  public class func get_binding() -> ext_texture_loader {
    return ext_texture_loader(
        load_texture: load_texture,
        destroy: destroy)
  }
  
  func loadTexture(resource_name: String) -> Texture {
    return Texture(resourceName: resource_name)
  }
}

private func load_texture(
    ref: UnsafeMutableRawPointer?,
    resourceNameExt: UnsafeMutableRawPointer?)
        -> UnsafeMutableRawPointer? {
          
  let textureLoader : TextureLoader = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeUnretainedValue()
  let resourceName = RustString(rawPointer: resourceNameExt)
  
  let texture = textureLoader.loadTexture(resource_name: resourceName.toString())
  
  return UnsafeMutableRawPointer(Unmanaged.passRetained(texture).toOpaque())
}

private func destroy(ref: UnsafeMutableRawPointer?) {
  let _ : TextureLoader
      = Unmanaged.fromOpaque(UnsafeRawPointer(ref!)).takeRetainedValue()
}

