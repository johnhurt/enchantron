//
//  Texture.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class Texture {

  let texture : SKTexture
  let size : CGSize
  
  init(texture: SKTexture) {
    self.texture = texture
    self.size = texture.size()
    texture.preload {
      print("Texture loaded size: \(self.texture.size())")
    }
    texture.filteringMode = SKTextureFilteringMode.linear
  }

  convenience init(resourceName: String) {
    self.init(texture: SKTexture(imageNamed: resourceName))
  }

  func getSubTexture(_ left: Int64, _ top: Int64, _ width: Int64, _ height: Int64) -> Texture {
    
    let tWidth = CGFloat(width) / self.size.width
    let tHeight = CGFloat(height) / self.size.height
    
    let tLeft = CGFloat(left) / self.size.width
    let tBottom = ( self.size.height - CGFloat(top) - CGFloat(height) ) / self.size.height
    
    let rect = CGRect(
        origin: CGPoint(x: tLeft, y: tBottom),
        size: CGSize(width: tWidth, height: tHeight))
    let result = Texture(texture: SKTexture(rect: rect, in: self.texture))
    return result
  }

  func getWidth() -> Int64 {
    return Int64(self.size.width)
  }
  
  func getHeight() -> Int64 {
    return Int64(self.size.height)
  }
  
  deinit {
    print("Dropping Texture")
  }
}
