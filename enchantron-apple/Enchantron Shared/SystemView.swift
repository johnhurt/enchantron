//
//  SystemView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation

class SystemView {
  let textureLoader : TextureLoader
  let viewport : Viewport
  
  init(textureLoader : TextureLoader, viewport : Viewport) {
    self.textureLoader = textureLoader
    self.viewport = viewport
  }
}
