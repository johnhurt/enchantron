//
//  MainMenu.swift
//  Enchantron
//
//  Created by Kevin Guthrie on 7/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class MainMenuView : BaseView {
    private static let MAX_WIDTH_FRAC : CGFloat = 0.5
    private static let HEIGHT_FRAC : CGFloat = 0.2
    private static let BUTTON_ASPECT_RATIO : CGFloat = 1.618
    
    override init(viewport: Viewport, device: MTLDevice) {
        super.init(viewport: viewport, device: device)
    }
    
    func transitionToGameView() {
        transitionTo(
            newView: GameView(viewport: viewport, device: device),
            binder: getContext().transitionToGameView)
    }
    
    override func localLayout(size: CGSize) {
        let maxHeight = size.height * MainMenuView.HEIGHT_FRAC
        let maxWidth = size.width * MainMenuView.MAX_WIDTH_FRAC
        
        let width = min(maxWidth, maxHeight * MainMenuView.BUTTON_ASPECT_RATIO)
        let height = width / MainMenuView.BUTTON_ASPECT_RATIO
        
    }
    
    deinit {
        print("Dropping MainMenuView")
    }
}

