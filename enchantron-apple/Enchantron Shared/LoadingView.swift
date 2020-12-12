//
//  LoadingView.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/16/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal
import MetalKit
import simd

class LoadingView : BaseView {
    private static let MAX_WIDTH_FRAC : CGFloat = 0.5
    private static let HEIGHT_FRAC : CGFloat = 0.2
    private static let BUTTON_ASPECT_RATIO : CGFloat = 1.618
    
    override init(viewport: Viewport, device: MTLDevice) {
        super.init(viewport: viewport, device: device)
    }
    
    
    func transitionToMainMenuView() {
        transitionTo(
            newView: MainMenuView(viewport: viewport, device: device),
            binder: getContext().transitionToMainMenuView)
    }
    
    deinit {
        print("Dropping Loading view")
    }
}
