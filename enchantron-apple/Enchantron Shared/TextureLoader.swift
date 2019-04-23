//
//  TextureLoader.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import SpriteKit

class TextureLoader {

  func loadTexture(_ resourceName: String) -> Texture {
    return Texture(resourceName: resourceName)
  }

  deinit {
    print("Dropping Texture Loader")
  }
}
