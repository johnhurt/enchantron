//
//  SystemInterop.swift
//  FourFours
//
//  Created by Kevin Guthrie on 8/19/18.
//  Copyright © 2018 Rook And Pawn Industries, Inc. All rights reserved.
//

import Foundation
import Metal

class SystemInterop {
    let resourceLoader : ResourceLoader
    let transitionService : TransitionService
    let device: MTLDevice
    private var screenSize = CGSize()
    private var screenSizeLock = NSLock()
    
    init(
        resourceLoader : ResourceLoader,
        transitionService: TransitionService,
        device: MTLDevice)
    {
        self.resourceLoader = resourceLoader
        self.transitionService = transitionService
        self.device = device
    }
    
    func getResourceLoader() -> ResourceLoader {
        return self.resourceLoader
    }
    
    func setScreenSize(_ screenSize: CGSize) {
        screenSizeLock.lock()
        self.screenSize = screenSize
        screenSizeLock.unlock()
    }
    
    func createNativeView() -> NativeView {
        screenSizeLock.lock()
        let newScreenSize = self.screenSize
        screenSizeLock.unlock()
        return NativeView(screenSize: newScreenSize, device: device)
    }
    
    func getTransitionService() -> TransitionService {
        return self.transitionService
    }
}
