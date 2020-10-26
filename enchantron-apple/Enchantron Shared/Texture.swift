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
  var anchorPoint : CGPoint = CGPoint(x: CGFloat(0), y: CGFloat(1))
  
  init(texture: SKTexture) {
    self.texture = texture
    self.texture.filteringMode = SKTextureFilteringMode.nearest
    self.size = texture.size()
    
    texture.preload {
      print("Texture loaded size: \(self.texture.size())")
    }
  }

  convenience init(resourceName: String) {
    let rawTexture = SKTexture(imageNamed: resourceName)
    self.init(texture: rawTexture)
  }

  func getSubTexture(_ left: Int64, _ top: Int64, _ width: Int64, _ height: Int64) -> Texture {
    
    let tWidth = CGFloat(width) / self.size.width * 0.99
    let tHeight = CGFloat(height) / self.size.height * 0.99
    
    let tLeft = CGFloat(left) / self.size.width + tWidth * 0.005
    let tBottom = ( self.size.height - CGFloat(top) - CGFloat(height) ) / self.size.height + tHeight * 0.005
    
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
  
  func setCenterRegistration(_ registerCenter: Bool) {
    self.anchorPoint = CGPoint(x: CGFloat(0.5), y: CGFloat(0.5))
  }
}
